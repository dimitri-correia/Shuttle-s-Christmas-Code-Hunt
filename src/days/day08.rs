use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use reqwest::Client;
use serde_json::Value;

pub fn get_day_8_router() -> Router {
    let client = Client::new();
    Router::new()
        .route("/weight/:poke_number", get(poke_weight))
        .route("/drop/:poke_number", get(drop))
        .with_state(client)
}

async fn drop(Path(poke_number): Path<i32>, State(client): State<Client>) -> (StatusCode, String) {
    let weight = get_weight_in_kilo(poke_number, client).await;
    let height: f64 = 10.0;
    let gravity: f64 = 9.825;
    let momentum = (gravity * height * 2.0).sqrt() * weight;

    (StatusCode::OK, momentum.to_string())
}

async fn poke_weight(
    Path(poke_number): Path<i32>,
    State(client): State<Client>,
) -> (StatusCode, String) {
    let weight = get_weight_in_kilo(poke_number, client).await;
    (StatusCode::OK, weight.to_string())
}

async fn get_weight_in_kilo(poke_number: i32, client: Client) -> f64 {
    let weight = if let Ok(response) = client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{poke_number}"))
        .send()
        .await
    {
        let data: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
        let weight_in_hectogram = data.get("weight").unwrap();
        let weight_in_kilogram = weight_in_hectogram.as_f64().unwrap() / 10.0;
        weight_in_kilogram
    } else {
        0.0
    };
    weight
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_8_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/weight/25").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(6.to_string());
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_8_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/drop/25").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(84.10707461325713.to_string());
    }
}
