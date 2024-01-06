use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::{
    ops::ControlFlow,
    sync::{atomic::AtomicBool, atomic::Ordering, Arc},
};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use futures::{stream::SplitSink, SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::{watch, RwLock};

#[derive(Debug, Clone)]
struct ChatAppState {
    room_channel: Arc<RwLock<HashMap<u32, watch::Sender<Message>>>>,
    views: Arc<AtomicUsize>,
}

pub fn get_day_19_router() -> Router {
    let chat_app_state = ChatAppState {
        room_channel: Arc::new(RwLock::new(HashMap::new())),
        views: Arc::new(AtomicUsize::new(0)),
    };
    let ping_game_router: Router = Router::new().route("/ws/ping", get(ping));
    let chat_app_router: Router = Router::new()
        .route("/reset", post(reset))
        .route("/views", get(view))
        .route("/ws/room/:room/user/:user", get(connect_to_a_room))
        .with_state(chat_app_state);
    Router::new()
        .nest("/", ping_game_router)
        .nest("/", chat_app_router)
}

async fn ping(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_ping)
}

async fn handle_ping(ws: WebSocket) {
    // started is independent for each connection
    let started = Arc::new(AtomicBool::new(false));
    let (sender, mut receiver) = ws.split();

    let sender = Arc::new(RwLock::new(sender));

    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_ping_message(msg, started.clone(), sender.clone())
                .await
                .is_break()
            {
                started.store(false, Ordering::Relaxed);
                break;
            }
        }
    });
}

async fn process_ping_message(
    message: Message,
    started: Arc<AtomicBool>,
    sender: Arc<RwLock<SplitSink<WebSocket, Message>>>,
) -> ControlFlow<(), ()> {
    if let Message::Text(s) = message {
        match s.as_str() {
            "serve" => {
                started.store(true, Ordering::Relaxed);
            }
            "ping" => {
                if started.load(Ordering::Relaxed)
                    && sender
                        .write()
                        .await
                        .send(Message::Text(String::from("pong")))
                        .await
                        .is_err()
                {
                    return ControlFlow::Break(());
                }
            }
            _ => {}
        }
    }
    ControlFlow::Continue(())
}

async fn reset(State(state): State<ChatAppState>) -> StatusCode {
    state.views.store(0, Ordering::Relaxed);
    StatusCode::OK
}

async fn view(State(state): State<ChatAppState>) -> impl IntoResponse {
    state.views.load(Ordering::Relaxed).to_string()
}

async fn connect_to_a_room(
    Path((room, user)): Path<(u32, String)>,
    ws: WebSocketUpgrade,
    State(state): State<ChatAppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_chat(socket, state, room, user))
}

async fn handle_chat(ws: WebSocket, state: ChatAppState, room: u32, user: String) {
    if !state.room_channel.read().await.contains_key(&room) {
        let (tx, _rx) = watch::channel(Message::Text("{}".to_string()));
        state.room_channel.write().await.insert(room, tx);
    }

    let (mut sender, mut receiver) = ws.split();

    let mut rx = state
        .room_channel
        .read()
        .await
        .get(&room)
        .unwrap()
        .subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(()) = rx.changed().await {
            let msg = rx.borrow().clone();
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let recv_user = user.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if process_chat_message(msg, room, recv_user.clone(), state.clone())
                    .await
                    .is_break()
                {
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

async fn process_chat_message(
    msg: Message,
    room: u32,
    user: String,
    state: ChatAppState,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(text) => {
            let msg = serde_json::from_str::<Value>(&text).unwrap();
            let message = msg.get("message").unwrap().as_str().unwrap();
            if message.len() > 128 {
                return ControlFlow::Continue(());
            }
            if msg.get("user").is_none() {
                let broadcast_msg = format!(r#"{{"user": "{}", "message": "{}"}}"#, user, message);

                if state
                    .room_channel
                    .write()
                    .await
                    .get(&room)
                    .unwrap()
                    .send(Message::Text(broadcast_msg))
                    .is_ok()
                {
                    let count = state
                        .room_channel
                        .write()
                        .await
                        .get(&room)
                        .unwrap()
                        .receiver_count();
                    state.views.fetch_add(count, Ordering::Relaxed);
                } else {
                    return ControlFlow::Break(());
                }
            }
        }
        Message::Close(_) => {
            return ControlFlow::Break(());
        }
        _ => {}
    }
    ControlFlow::Continue(())
}

// TODO tests not working
#[cfg(test)]
mod tests {
    use std::{
        future::IntoFuture,
        net::{Ipv4Addr, SocketAddr},
    };

    use tokio::net::TcpStream;
    use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};

    use super::*;

    #[tokio::test]
    async fn task1() {
        let listener = tokio::net::TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
            .await
            .unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(axum::serve(listener, get_day_19_router()).into_future());

        let (mut socket, _response) =
            tokio_tungstenite::connect_async(format!("ws://{addr}/ws/ping"))
                .await
                .unwrap();

        verify_message(&mut socket, "ping", "pong").await;
    }

    async fn verify_message(
        socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        send: &str,
        expected: &str,
    ) {
        socket.send(tungstenite::Message::text(send)).await.unwrap();

        let msg = match socket.next().await.unwrap().unwrap() {
            tungstenite::Message::Text(msg) => msg,
            other => panic!("expected a text message but got {other:?}"),
        };

        assert_eq!(msg, expected);
    }
}
