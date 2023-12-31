use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MyState {
    pub pool: sqlx::PgPool,
}

#[derive(Deserialize, Serialize)]
pub struct Order {
    pub id: i32,
    pub region_id: i32,
    pub gift_name: String,
    pub quantity: i32,
}
