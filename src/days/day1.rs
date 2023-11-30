use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

pub fn get_day_1_router() -> Router {
    Router::new().route("/okok", get(hello_world))
}

async fn hello_world() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}
