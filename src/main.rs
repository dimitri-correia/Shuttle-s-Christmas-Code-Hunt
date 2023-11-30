use axum::http::StatusCode;
use axum::{routing::get, Router};

async fn hello_world() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error));

    Ok(router.into())
}
