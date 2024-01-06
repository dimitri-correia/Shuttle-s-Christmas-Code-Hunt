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

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;
    use serial_test::serial;
    use sqlx::PgPool;

    use super::*;

    async fn setup_test_server() -> TestServer {
        let pool = PgPool::connect("postgresql://dim:dim@localhost:3269/db")
            .await
            .expect("Failed to connect to the database for testing");

        let db = MyState { pool };
        let app = get_day_13_router(db);

        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    #[serial]
    async fn task1() {
        // Run the application for testing.
        let server = setup_test_server().await;

        // Send the request.
        let response = server.get("/sql").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(20231213.to_string());
    }

    #[tokio::test]
    #[serial]
    async fn task2() {
        // Run the application for testing.
        let server = setup_test_server().await;

        // Send the request.
        let response = server.post("/reset").await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server
            .post("/orders")
            .json(
                &json!([{"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
    {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
    {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
    {"id":4,"region_id":4,"gift_name":"Board Game","quantity":10},
    {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
    {"id":6,"region_id":3,"gift_name":"Toy Train","quantity":3}]),
            )
            .await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server.get("/orders/total").await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({"total":44}));
    }

    #[tokio::test]
    #[serial]
    async fn task3() {
        // Run the application for testing.
        let server = setup_test_server().await;

        // Send the request.
        let response = server.post("/reset").await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server
            .post("/orders")
            .json(
                &json!([{"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
    {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
    {"id":3,"region_id":3,"gift_name":"Toy Train","quantity":4}]),
            )
            .await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server.get("/orders/popular").await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({"popular":"Toy Train"}));
    }
}
