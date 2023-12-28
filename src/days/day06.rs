use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub fn get_day_6_router() -> Router {
    Router::new().route("/", post(count_elves))
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Res {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn count_elves(body: String) -> impl IntoResponse {
    let elf = body.matches("elf").count();
    let shelf = body.matches("shelf").count();
    let elf_on_a_shelf = body
        .chars()
        .collect::<Vec<_>>()
        .windows("elf on a shelf".len())
        .filter(|window| window.iter().collect::<String>() == "elf on a shelf")
        .count();
    let shelf_with_no_elf_on_it = shelf - elf_on_a_shelf;

    Json(Res {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
    .into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_6_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .text(
                "The mischievous elf peeked out from behind the toy workshop,
                and another elf joined in the festive dance.
                Look, there is also an elf on that shelf!",
            )
            .await;

        response.assert_status(StatusCode::OK);

        assert_eq!(response.json::<Res>().elf, 4);
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_6_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .text(
                "there is an elf on a shelf on an elf.
                there is also another shelf in Belfast.",
            )
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
            "elf": 5,
            "elf on a shelf": 1,
            "shelf with no elf on it": 1
        }));
    }
}
