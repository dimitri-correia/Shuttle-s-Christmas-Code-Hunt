use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

pub fn get_day_1_router() -> Router {
    Router::new().route("/*l_nums", get(cube_the_bits))
}

async fn cube_the_bits(Path(l_nums): Path<String>) -> (StatusCode, String) {
    let res = l_nums
        .split('/')
        .map(|n| n.parse::<i32>().unwrap())
        .fold(0, |acc, v| acc ^ v)
        .pow(3);

    (StatusCode::OK, res.to_string())
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_1_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/4/8").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(1728.to_string());
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_1_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("4/5/8/10").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(27.to_string());
    }
}
