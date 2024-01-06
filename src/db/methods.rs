use serde_json::Value;
use sqlx::Row;

use crate::db::structs::{MyState, Order, Region};

pub async fn reset(db: MyState) {
    sqlx::query(include_str!("../../migrations/1_drop_orders.sql"))
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query(include_str!("../../migrations/2_create_orders.sql"))
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query(include_str!("../../migrations/3_drop_regions.sql"))
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query(include_str!("../../migrations/4_create_regions.sql"))
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

pub async fn insert_regions(db: MyState, data: Vec<Region>) {
    for region in data {
        sqlx::query("INSERT INTO regions (id, name) VALUES ($1, $2)")
            .bind(region.id)
            .bind(region.name)
            .execute(&db.pool)
            .await
            .unwrap();
    }
}

pub async fn get_number_region(db: MyState) -> Vec<(String, i64)> {
    sqlx::query_as(
        "SELECT r.name AS region, SUM(o.quantity)
FROM regions r
LEFT JOIN orders o ON r.id = o.region_id
GROUP BY r.name
HAVING SUM(o.quantity) IS NOT NULL
ORDER BY r.name;",
    )
    .fetch_all(&db.pool)
    .await
    .unwrap()
}

pub async fn get_top_gifts(db: MyState, nb_gifts: i32) -> Vec<(String, Option<String>)> {
    if nb_gifts == 0 {
        return get_no_top_gifts(&db).await;
    }
    sqlx::query_as(
        "WITH RankedGifts AS (
    SELECT
        r.name AS region,
        o.gift_name,
        ROW_NUMBER() OVER (PARTITION BY r.id ORDER BY -SUM(o.quantity), o.gift_name) AS row_num
    FROM
        regions r
            LEFT JOIN
        orders o ON r.id = o.region_id
    GROUP BY
        r.name, o.gift_name, r.id
)
SELECT
    region,
    STRING_AGG(gift_name, ', ') AS all_gifts
FROM
    RankedGifts
WHERE
    row_num <= $1
GROUP BY
    region
ORDER BY
    region;
",
    )
    .bind(nb_gifts)
    .fetch_all(&db.pool)
    .await
    .unwrap()
}

async fn get_no_top_gifts(db: &MyState) -> Vec<(String, Option<String>)> {
    sqlx::query_as("SELECT name, null FROM regions ORDER BY name")
        .fetch_all(&db.pool)
        .await
        .unwrap()
}
