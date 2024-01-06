use axum::extract::{Path, State};
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
        .route("/regions/top_list/:number", get(get_top_list))
        .with_state(db)
}

async fn reset(State(db): State<MyState>) -> StatusCode {
    db::methods::reset(db).await;

    StatusCode::OK
}

async fn insert_orders(State(db): State<MyState>, Json(data): Json<Vec<Order>>) -> StatusCode {
    db::methods::insert_orders(db, data).await;

    StatusCode::OK
}

async fn insert_regions(State(db): State<MyState>, Json(data): Json<Vec<Region>>) -> StatusCode {
    db::methods::insert_regions(db, data).await;

    StatusCode::OK
}

#[derive(Deserialize, Serialize)]
struct Total {
    region: String,
    total: i64,
}

async fn get_number_region(State(db): State<MyState>) -> (StatusCode, Json<Vec<Total>>) {
    (
        StatusCode::OK,
        Json(
            db::methods::get_number_region(db)
                .await
                .into_iter()
                .map(|(region, total)| Total { region, total })
                .collect(),
        ),
    )
}

#[derive(Deserialize, Serialize, Debug)]
struct TopGifts {
    region: String,
    top_gifts: Vec<String>,
}

async fn get_top_list(
    Path(number): Path<i32>,
    State(db): State<MyState>,
) -> (StatusCode, Json<Vec<TopGifts>>) {
    (
        StatusCode::OK,
        Json(
            db::methods::get_top_gifts(db, number)
                .await
                .into_iter()
                .map(|(region, top_gifts)| TopGifts {
                    region,
                    top_gifts: get_top_gifts(top_gifts),
                })
                .collect(),
        ),
    )
}

fn get_top_gifts(s: Option<String>) -> Vec<String> {
    s.map(|gifts| {
        gifts
            .split(',')
            .map(|gift| gift.trim().to_string())
            .collect()
    })
    .unwrap_or_default()
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
        let app = get_day_18_router(db);

        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    #[serial]
    async fn task1() {
        // Run the application for testing.
        let server = setup_test_server().await;

        // Send the request.
        let response = server.post("/reset").await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server
            .post("/regions")
            .json(&json!([
    {"id":1,"name":"North Pole"},
    {"id":2,"name":"Europe"},
    {"id":3,"name":"North America"},
    {"id":4,"name":"South America"},
    {"id":5,"name":"Africa"},
    {"id":6,"name":"Asia"},
    {"id":7,"name":"Oceania"}]))
            .await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server
            .post("/orders")
            .json(&json!([
    {"id":1,"region_id":2,"gift_name":"Board Game","quantity":5},
    {"id":2,"region_id":2,"gift_name":"Origami Set","quantity":8},
    {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
    {"id":4,"region_id":4,"gift_name":"Teddy Bear","quantity":10},
    {"id":5,"region_id":2,"gift_name":"Yarn Ball","quantity":6},
    {"id":6,"region_id":3,"gift_name":"Art Set","quantity":3},
    {"id":7,"region_id":5,"gift_name":"Robot Lego Kit","quantity":5},
    {"id":8,"region_id":6,"gift_name":"Drone","quantity":9}]))
            .await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server.get("/regions/total").await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!([
  {"region":"Africa","total":5},
  {"region":"Asia","total":9},
  {"region":"Europe","total":19},
  {"region":"North America","total":15},
  {"region":"South America","total":10}]));
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
            .post("/regions")
            .json(&json!([{"id":1,"name":"North Pole"},
    {"id":2,"name":"South Pole"},
    {"id":3,"name":"Kiribati"},
    {"id":4,"name":"Baker Island"}]))
            .await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server
            .post("/orders")
            .json(&json!([
    {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
    {"id":2,"region_id":2,"gift_name":"Toy Train","quantity":3},
    {"id":3,"region_id":2,"gift_name":"Doll","quantity":8},
    {"id":4,"region_id":3,"gift_name":"Toy Train","quantity":3},
    {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
    {"id":6,"region_id":3,"gift_name":"Action Figure","quantity":12},
    {"id":7,"region_id":4,"gift_name":"Board Game","quantity":10},
    {"id":8,"region_id":3,"gift_name":"Teddy Bear","quantity":1},
    {"id":9,"region_id":3,"gift_name":"Teddy Bear","quantity":2}]))
            .await;

        response.assert_status(StatusCode::OK);

        // Send the request.
        let response = server.get("/regions/top_list/2").await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!([
  {"region":"Baker Island","top_gifts":["Board Game"]},
  {"region":"Kiribati","top_gifts":["Action Figure","Teddy Bear"]},
  {"region":"North Pole","top_gifts":[]},
  {"region":"South Pole","top_gifts":["Doll","Toy Train"]}]));
    }
}
