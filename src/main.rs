use axum::http::{StatusCode, Uri};
use axum::Router;
use tracing::info;

use crate::days::day0::get_day_0_router;
use crate::days::day1::get_day_1_router;
use crate::days::day4::get_day_4_router;

mod days;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(get_day_0_router())
        .nest("/1", get_day_1_router())
        .nest("/4", get_day_4_router())
        .fallback(fallback);

    info!("App ok");

    Ok(router.into())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
