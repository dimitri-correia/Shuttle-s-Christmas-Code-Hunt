use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

pub fn get_day_4_router() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}

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

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_4_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/strength")
            .json(&json!([
                { "name": "Dasher", "strength": 5 },
                { "name": "Dancer", "strength": 6 },
                { "name": "Prancer", "strength": 4 },
                { "name": "Vixen", "strength": 7 }
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text(22.to_string());
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_4_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/contest")
            .json(&json!([
                {
                    "name": "Dasher",
                    "strength": 5,
                    "speed": 50.4,
                    "height": 80,
                    "antler_width": 36,
                    "snow_magic_power": 9001,
                    "favorite_food": "hay",
                    "cAnD13s_3ATeN-yesT3rdAy": 2
                },
                {
                    "name": "Dancer",
                    "strength": 6,
                    "speed": 48.2,
                    "height": 65,
                    "antler_width": 37,
                    "snow_magic_power": 4004,
                    "favorite_food": "grass",
                    "cAnD13s_3ATeN-yesT3rdAy": 5
                }
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
          "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
          "tallest": "Dasher is standing tall with his 36 cm wide antlers",
          "magician": "Dasher could blast you away with a snow magic power of 9001",
          "consumer": "Dancer ate lots of candies, but also some grass"
        }));
    }
}
