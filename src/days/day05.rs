use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;

pub fn get_day_5_router() -> Router {
    Router::new().route("/", post(slicing_the_loop))
}

#[derive(Deserialize, Debug)]
struct Pagination {
    #[serde(default = "default_offset")]
    offset: usize,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default = "default_split")]
    split: usize,
}

fn default_offset() -> usize {
    0
}

fn default_limit() -> usize {
    0
}

fn default_split() -> usize {
    0
}

async fn slicing_the_loop(
    pagination: Query<Pagination>,
    names: Json<Vec<String>>,
) -> impl IntoResponse {
    let start = pagination.offset;

    let end = if pagination.limit == 0 {
        names.len()
    } else {
        start + pagination.limit
    };

    if pagination.split == 0 {
        Json(&names[start..end]).into_response()
    } else {
        Json(
            &names[start..end]
                .chunks(pagination.split)
                .collect::<Vec<_>>(),
        )
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_5_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .add_query_param("offset", 3)
            .add_query_param("limit", 5)
            .json(&json!([
                "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper", "Lucas",
                "Stella", "Mason", "Olivia"
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!(["Owen", "Lily", "Ethan", "Zoe", "Nolan"]));
    }

    #[tokio::test]
    async fn task2_1() {
        let app = get_day_5_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .add_query_param("split", 4)
            .json(&json!([
                "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper", "Lucas",
                "Stella", "Mason", "Olivia"
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!([
            ["Ava", "Caleb", "Mia", "Owen"],
            ["Lily", "Ethan", "Zoe", "Nolan"],
            ["Harper", "Lucas", "Stella", "Mason"],
            ["Olivia"]
        ]));
    }

    #[tokio::test]
    async fn task2_2() {
        let app = get_day_5_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .add_query_param("offset", 5)
            .add_query_param("split", 2)
            .json(&json!([
                "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper", "Lucas",
                "Stella", "Mason", "Olivia"
            ]))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!([
            ["Ethan", "Zoe"],
            ["Nolan", "Harper"],
            ["Lucas", "Stella"],
            ["Mason", "Olivia"]
        ]));
    }
}
