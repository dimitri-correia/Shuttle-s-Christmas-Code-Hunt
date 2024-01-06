use serde_json::Value;
use sqlx::Row;

use crate::db::structs::{MyState, Order};

pub async fn reset(db: MyState) {
    sqlx::query(include_str!("../../migrations/1_drop_orders.sql"))
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query(include_str!("../../migrations/2_create_orders.sql"))
        .execute(&db.pool)
        .await
        .unwrap();
}

pub async fn insert_orders(db: MyState, data: Vec<Order>) {
    for order in data {
        sqlx::query(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(order.gift_name)
        .bind(order.quantity)
        .execute(&db.pool)
        .await
        .unwrap();
    }
}

pub async fn get_number_order(db: MyState) -> i64 {
    match sqlx::query("SELECT SUM(quantity) FROM orders")
        .fetch_one(&db.pool)
        .await
    {
        Ok(row) => row.get::<i64, _>("sum"),
        Err(_) => 0,
    }
}

pub async fn get_most_popular_order(db: MyState) -> Value {
    match sqlx::query("SELECT gift_name, SUM(quantity) as total_quantity FROM orders GROUP BY gift_name ORDER BY total_quantity DESC LIMIT 1")
        .fetch_one(&db.pool)
        .await
    {
        Ok(row) => Value::from(row.get::<String, _>("gift_name")),
        Err(_) => Value::Null,
    }
}
