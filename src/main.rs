use axum::Router;

use crate::days::day0::get_day_0_router;

mod days;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().merge(get_day_0_router());

    Ok(router.into())
}
