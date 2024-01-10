use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use uuid::Uuid;

pub fn get_day_12_router() -> Router {
    let shared_state = Default::default();
    Router::new()
        .route("/save/:packet_id", post(save))
        .route("/load/:packet_id", get(load))
        .route("/ulids", post(ulids))
        .route("/ulids/:week_day", post(analyze_ulids))
        .with_state(shared_state)
}

#[derive(Default)]
struct AppState {
    timekeeper: HashMap<String, Instant>,
}

type SharedState = Arc<RwLock<AppState>>;

async fn save(Path(packet_id): Path<String>, State(state): State<SharedState>) -> StatusCode {
    let timekeeper = &mut state.write().unwrap().timekeeper;
    timekeeper.insert(packet_id, Instant::now());
    StatusCode::OK
}

async fn load(
    Path(packet_id): Path<String>,
    State(state): State<SharedState>,
) -> (StatusCode, String) {
    let timekeeper = &state.read().unwrap().timekeeper;
    let time = timekeeper.get(&packet_id);
    if let Some(time) = time {
        (StatusCode::OK, time.elapsed().as_secs().to_string())
    } else {
        (StatusCode::BAD_REQUEST, "No associated data".to_string())
    }
}

async fn ulids(Json(payload): Json<Vec<String>>) -> (StatusCode, Json<Vec<String>>) {
    let new_ids: Vec<String> = payload
        .iter()
        .map(|old_id| Uuid::from_bytes(Ulid::from_string(old_id).unwrap().into()).to_string())
        .rev()
        .collect();
    (StatusCode::OK, Json(new_ids))
}

#[derive(Serialize, Deserialize)]
struct Lsb {
    #[serde(rename = "christmas eve")]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename = "in the future")]
    in_future: usize,
    #[serde(rename = "LSB is 1")]
    lsb: usize,
}

async fn analyze_ulids(
    Path(week_day): Path<u32>,
    Json(payload): Json<Vec<String>>,
) -> (StatusCode, Json<Lsb>) {
    let (lsb, christmas_eve, weekday, in_future) = payload
        .iter()
        .map(|id| Ulid::from_string(id).unwrap())
        .fold((0, 0, 0, 0), |acc, ulid| {
            let lsb_count = if (ulid.0 & 1) != 0 { acc.0 + 1 } else { acc.0 };

            let date = DateTime::<Utc>::from(ulid.datetime());

            let christmas_eve_count = if date.month() == 12 && date.day() == 24 {
                acc.1 + 1
            } else {
                acc.1
            };

            let weekday_count = if date.weekday().num_days_from_monday() == week_day {
                acc.2 + 1
            } else {
                acc.2
            };

            let in_future_count = if Utc::now() < date { acc.3 + 1 } else { acc.3 };

            (
                lsb_count,
                christmas_eve_count,
                weekday_count,
                in_future_count,
            )
        });

    (
        StatusCode::OK,
        Json(Lsb {
            christmas_eve,
            weekday,
            in_future,
            lsb,
        }),
    )
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_12_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let saved_txt = "packet20231212";
        let sleeping_time_in_s = 1;

        assert_save(&server, &saved_txt).await;

        sleep(Duration::from_secs(sleeping_time_in_s));
        assert_load(&server, &saved_txt, sleeping_time_in_s).await;

        sleep(Duration::from_secs(sleeping_time_in_s));
        assert_load(&server, &saved_txt, sleeping_time_in_s * 2).await;

        assert_save(&server, &saved_txt).await;
        assert_load(&server, &saved_txt, 0).await;
    }

    async fn assert_load(server: &TestServer, saved_txt: &&str, sleeping_time: u64) {
        let response_load = server.get(&format!("/load/{saved_txt}")).await;
        response_load.assert_status(StatusCode::OK);
        response_load.assert_text(sleeping_time.to_string());
    }

    async fn assert_save(server: &TestServer, saved_txt: &&str) {
        let response_save = server.post(&format!("/save/{saved_txt}")).await;
        response_save.assert_status(StatusCode::OK);
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_12_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/ulids")
            .json(&json!([
                "01BJQ0E1C3Z56ABCD0E11HYX4M",
                "01BJQ0E1C3Z56ABCD0E11HYX5N",
                "01BJQ0E1C3Z56ABCD0E11HYX6Q",
                "01BJQ0E1C3Z56ABCD0E11HYX7R",
                "01BJQ0E1C3Z56ABCD0E11HYX8P"
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!([
            "015cae07-0583-f94c-a5b1-a070431f7516",
            "015cae07-0583-f94c-a5b1-a070431f74f8",
            "015cae07-0583-f94c-a5b1-a070431f74d7",
            "015cae07-0583-f94c-a5b1-a070431f74b5",
            "015cae07-0583-f94c-a5b1-a070431f7494"
        ]));
    }

    #[tokio::test]
    async fn task3() {
        let app = get_day_12_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/ulids/5")
            .json(&json!([
                "00WEGGF0G0J5HEYXS3D7RWZGV8",
                "76EP4G39R8JD1N8AQNYDVJBRCF",
                "018CJ7KMG0051CDCS3B7BFJ3AK",
                "00Y986KPG0AMGB78RD45E9109K",
                "010451HTG0NYWMPWCEXG6AJ8F2",
                "01HH9SJEG0KY16H81S3N1BMXM4",
                "01HH9SJEG0P9M22Z9VGHH9C8CX",
                "017F8YY0G0NQA16HHC2QT5JD6X",
                "03QCPC7P003V1NND3B3QJW72QJ"
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
          "christmas eve": 3,
          "weekday": 1,
          "in the future": 2,
          "LSB is 1": 5
        }));
    }
}
