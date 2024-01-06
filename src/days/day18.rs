use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::db;
use crate::db::structs::{MyState, Order, Region};

pub fn get_day_18_router(db: MyState) -> Router {
    Router::new()
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/regions", post(insert_regions))
        .route("/regions/total", get(get_number_region))
        .with_state(db)
}

async fn reset(State(db): State<MyState>) -> StatusCode {
    db::methods::reset(db).await;

    StatusCode::OK
}

async fn insert_orders(State(db): State<MyState>, Json(data): Json<Vec<Order>>) -> StatusCode {
    dbg!(&data);
    db::methods::insert_orders(db, data).await;

    StatusCode::OK
}

async fn insert_regions(State(db): State<MyState>, Json(data): Json<Vec<Region>>) -> StatusCode {
    dbg!(&data);
    db::methods::insert_regions(db, data).await;

    StatusCode::OK
}

#[derive(Deserialize, Serialize)]
struct Total {
    region: String,
    total: i64,
}

async fn get_number_region(State(db): State<MyState>) -> (StatusCode, Json<Vec<Total>>) {
    let tot: Vec<(String, i64)> = db::methods::get_number_region(db).await;

    dbg!(&tot);

    (
        StatusCode::OK,
        Json(
            tot.into_iter()
                .map(|(region, total)| Total { region, total })
                .collect(),
        ),
    )
}
