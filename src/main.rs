use axum::http::{StatusCode, Uri};
use axum::Router;
use tracing::info;

use crate::days::day00::get_day_0_router;
use crate::days::day01::get_day_1_router;
use crate::days::day04::get_day_4_router;
use crate::days::day05::get_day_5_router;
use crate::days::day06::get_day_6_router;
use crate::days::day07::get_day_7_router;
use crate::days::day08::get_day_8_router;
use crate::days::day11::get_day_11_router;

mod days;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(get_day_0_router())
        .nest("/1", get_day_1_router())
        .nest("/4", get_day_4_router())
        .nest("/5", get_day_5_router())
        .nest("/6", get_day_6_router())
        .nest("/7", get_day_7_router())
        .nest("/8", get_day_8_router())
        .nest("/11", get_day_11_router())
        .fallback(fallback);

    info!("App ok");

    Ok(router.into())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
