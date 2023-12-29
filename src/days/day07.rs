use std::collections::HashMap;

use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, Value};

pub fn get_day_7_router() -> Router {
    Router::new()
        .route("/decode", get(decode_cookie))
        .route("/bake", get(bake))
}

type Recipe = HashMap<String, Value>;

async fn decode_cookie(jar: CookieJar) -> Json<Recipe> {
    let encoded_recipe = jar.get("recipe").unwrap().value();
    let decoded_bytes = general_purpose::STANDARD
        .decode(encoded_recipe)
        .expect("Failed to decode Base64");
    let decoded_json: Recipe = from_slice(&decoded_bytes).expect("Failed to parse JSON");

    Json(decoded_json)
}

type Ingredients = HashMap<String, usize>;

#[derive(Serialize, Deserialize, Debug)]
struct BakeResult {
    cookies: usize,
    pantry: Ingredients,
}

#[derive(Serialize, Deserialize, Debug)]
struct BakeInstructions {
    recipe: Ingredients,
    pantry: Ingredients,
}

async fn bake(jar: CookieJar) -> Json<BakeResult> {
    let encoded_recipe = jar.get("recipe").unwrap().value();
    let decoded_bytes = general_purpose::STANDARD
        .decode(encoded_recipe)
        .expect("Failed to decode Base64");
    let bake_instructions: BakeInstructions =
        from_slice::<BakeInstructions>(&decoded_bytes).expect("Failed to parse JSON");

    let cookies = bake_instructions
        .recipe
        .iter()
        .filter_map(|(item, qtt)| {
            if qtt > &0 {
                Some(bake_instructions.pantry.get(item).unwrap_or(&0_usize) / qtt)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    let pantry = if cookies == 0 {
        bake_instructions.pantry
    } else {
        bake_instructions
            .pantry
            .iter()
            .map(|(item, qtt)| {
                (
                    item.clone(),
                    qtt - cookies * bake_instructions.recipe.get(item).unwrap_or(&0_usize),
                )
            })
            .collect::<Ingredients>()
    };

    dbg!(&cookies, &pantry);

    Json(BakeResult { cookies, pantry })
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_extra::extract::cookie::Cookie;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_7_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/decode")
            .add_cookie(Cookie::new(
                "recipe",
                "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
            ))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({"flour":100,"chocolate chips":20}));
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_7_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/bake")
            .add_cookie(Cookie::new(
                "recipe",
                "eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
            ))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
          "cookies": 4,
          "pantry": {
            "flour": 5,
            "sugar": 307,
            "butter": 2002,
            "baking powder": 825,
            "chocolate chips": 257
          }
        }));
    }

    #[tokio::test]
    async fn task3() {
        let app = get_day_7_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/bake")
            .add_cookie(Cookie::new(
                "recipe",
                "eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==",
            ))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
          "cookies": 0,
          "pantry": {
            "cobblestone": 64,
            "stick": 4
          }
        }));
    }
}
