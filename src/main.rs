use axum::Router;
use tracing::info;

use crate::days::day0::get_day_0_router;
use crate::days::day1::get_day_1_router;

mod days;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(get_day_0_router())
        .merge(get_day_1_router());

    info!("App ok");

    Ok(router.into())
}
