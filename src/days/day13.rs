use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db;
use crate::db::structs::{MyState, Order};

pub fn get_day_13_router(db: MyState) -> Router {
    Router::new()
        .route("/sql", get(sql_20231213))
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/orders/total", get(get_number_order))
        .route("/orders/popular", get(get_popular_order))
        .with_state(db)
}

#[derive(Debug, sqlx::FromRow)]
struct Number {
    number: i32,
}

async fn sql_20231213(State(db): State<MyState>) -> (StatusCode, String) {
    let number = sqlx::query_as::<_, Number>("SELECT 20231213 number")
        .fetch_one(&db.pool)
        .await
        .unwrap();

    (StatusCode::OK, number.number.to_string())
}

async fn reset(State(db): State<MyState>) -> StatusCode {
    db::methods::reset(db).await;

    StatusCode::OK
}

async fn insert_orders(State(db): State<MyState>, Json(data): Json<Vec<Order>>) -> StatusCode {
    db::methods::insert_orders(db, data).await;

    StatusCode::OK
}

#[derive(Deserialize, Serialize)]
struct Total {
    total: i64,
}

async fn get_number_order(State(db): State<MyState>) -> (StatusCode, Json<Total>) {
    let total: i64 = db::methods::get_number_order(db).await;

    (StatusCode::OK, Json(Total { total }))
}

#[derive(Deserialize, Serialize, Debug)]
struct Popular {
    popular: Value,
}

async fn get_popular_order(State(db): State<MyState>) -> (StatusCode, Json<Popular>) {
    let popular: Value = db::methods::get_most_popular_order(db).await;

    (StatusCode::OK, Json(Popular { popular }))
}
