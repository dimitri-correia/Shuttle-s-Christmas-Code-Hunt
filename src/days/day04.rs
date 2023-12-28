use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Reindeer {
    name: String,
    strength: f32,
    speed: f32,
    height: f32,
    antler_width: f32,
    snow_magic_power: f32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: f32,
}
#[derive(Deserialize, Debug)]
struct ReindeerSimple {
    strength: u32,
}

#[derive(Serialize, Debug)]
struct ReindeerResult {
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn strength(Json(reindeer_list): Json<Vec<ReindeerSimple>>) -> (StatusCode, String) {
    let sum: u32 = reindeer_list
        .iter()
        .map(|reindeer| -> u32 { reindeer.strength })
        .sum();
    (StatusCode::OK, sum.to_string())
}

async fn contest(Json(reindeer_list): Json<Vec<Reindeer>>) -> Json<ContestResult> {
    let fastest = get_reindeer_result(&reindeer_list, |r| r.speed);
    let fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength, fastest.name
    );
    let tallest = get_reindeer_result(&reindeer_list, |r| r.height);
    let tallest = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest.name, tallest.antler_width
    );
    let magician = get_reindeer_result(&reindeer_list, |r| r.snow_magic_power);
    let magician = format!(
        "{} could blast you away with a snow magic power of {}",
        magician.name, magician.snow_magic_power
    );
    let consumer = get_reindeer_result(&reindeer_list, |r| r.candies_eaten_yesterday);
    let consumer = format!(
        "{} ate lots of candies, but also some {}",
        consumer.name, consumer.favorite_food
    );

    Json(ContestResult {
        fastest,
        tallest,
        magician,
        consumer,
    })
}

fn get_reindeer_result<F>(reindeer_list: &[Reindeer], key_fn: F) -> &Reindeer
where
    F: Fn(&Reindeer) -> f32,
{
    reindeer_list
        .iter()
        .max_by_key(|&r| OrderedFloat(key_fn(r)))
        .unwrap()
}

pub fn get_day_4_router() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}
