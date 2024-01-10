use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

pub fn get_day_0_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
}

async fn hello_world() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
