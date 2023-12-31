use sqlx::Row;

use crate::db::structs::{MyState, Order};

pub async fn reset(db: MyState) {
    sqlx::query(include_str!("../../migrations/1_schema.sql"))
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

pub async fn get_most_popular_order(db: MyState) -> String {
    // match sqlx::query("SELECT MAX(quantity) FROM orders")
    //     .fetch_one(&db.pool)
    //     .await
    // {
    //     Ok(row) => row.get::<str, _>("max").to_string(),
    //     Err(_) => "null".to_string(),
    // }
    "null".to_string()
}
