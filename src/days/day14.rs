use axum::routing::post;
use axum::{Json, Router};

pub fn get_day_14_router() -> Router {
    Router::new()
        .route("/unsafe", post(unsafe_rendering))
        .route("/safe", post(safe_rendering))
}

#[derive(serde::Deserialize, Debug)]
struct SimpleBody {
    content: String,
}

fn html_boilerplate(html: String) -> String {
    format!(
        "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {html}
  </body>
</html>"
    )
}

async fn unsafe_rendering(Json(payload): Json<SimpleBody>) -> String {
    html_boilerplate(payload.content)
}

async fn safe_rendering(Json(payload): Json<SimpleBody>) -> String {
    html_boilerplate(html_escape::encode_double_quoted_attribute(&payload.content).to_string())
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_14_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/unsafe")
            .json(&json!({"content": "<h1>Welcome to the North Pole!</h1>"}))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text(
            "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    <h1>Welcome to the North Pole!</h1>
  </body>
</html>",
        );
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_14_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/safe")
            .json(&json!({"content": "<script>alert(\"XSS Attack!\")</script>"}))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text(
            "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    &lt;script&gt;alert(&quot;XSS Attack!&quot;)&lt;/script&gt;
  </body>
</html>",
        );
    }
}
