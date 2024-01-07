use axum::http::{StatusCode, Uri};
use axum::Router;
use sqlx::PgPool;
use tracing::info;

use crate::days::day00::get_day_0_router;
use crate::days::day01::get_day_1_router;
use crate::days::day04::get_day_4_router;
use crate::days::day05::get_day_5_router;
use crate::days::day06::get_day_6_router;
use crate::days::day07::get_day_7_router;
use crate::days::day08::get_day_8_router;
use crate::days::day11::get_day_11_router;
use crate::days::day12::get_day_12_router;
use crate::days::day13::get_day_13_router;
use crate::days::day14::get_day_14_router;
use crate::days::day15::get_day_15_router;
use crate::days::day18::get_day_18_router;
use crate::days::day19::get_day_19_router;
use crate::days::day20::get_day_20_router;
use crate::db::structs::MyState;

mod days;
mod db;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let db = init_db(pool).await;

    let router = Router::new()
        .merge(get_day_0_router())
        .nest("/1", get_day_1_router())
        .nest("/4", get_day_4_router())
        .nest("/5", get_day_5_router())
        .nest("/6", get_day_6_router())
        .nest("/7", get_day_7_router())
        .nest("/8", get_day_8_router())
        .nest("/11", get_day_11_router())
        .nest("/12", get_day_12_router())
        .nest("/13", get_day_13_router(db.clone()))
        .nest("/14", get_day_14_router())
        .nest("/15", get_day_15_router())
        .nest("/18", get_day_18_router(db.clone()))
        .nest("/19", get_day_19_router())
        .nest("/20", get_day_20_router())
        .fallback(fallback);

    info!("App ok");

    Ok(router.into())
}

async fn init_db(pool: PgPool) -> MyState {
    sqlx::migrate!().run(&pool).await.unwrap();

    MyState { pool }
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
