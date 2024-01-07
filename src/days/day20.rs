use axum::body::Bytes;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use bytes::Buf;
use tar::Archive;

pub fn get_day_20_router() -> Router {
    Router::new()
        .route("/archive_files", post(archive_files))
        .route("/archive_files_size", post(archive_files_size))
}

async fn archive_files(body: Bytes) -> (StatusCode, String) {
    (
        StatusCode::OK,
        Archive::new(body.reader())
            .entries()
            .unwrap()
            .count()
            .to_string(),
    )
}

async fn archive_files_size(body: Bytes) -> (StatusCode, String) {
    (
        StatusCode::OK,
        Archive::new(body.reader())
            .entries()
            .unwrap()
            .map(|file| file.unwrap().size())
            .sum::<u64>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1a() {
        let app = get_day_20_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let file_bytes = Bytes::from(include_bytes!("../../assets/northpole20231220.tar").to_vec());

        // Send the request.
        let response = server.post("/archive_files").bytes(file_bytes).await;

        response.assert_status(StatusCode::OK);

        response.assert_text(6.to_string());
    }

    #[tokio::test]
    async fn task1b() {
        let app = get_day_20_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let file_bytes = Bytes::from(include_bytes!("../../assets/northpole20231220.tar").to_vec());

        // Send the request.
        let response = server.post("/archive_files_size").bytes(file_bytes).await;

        response.assert_status(StatusCode::OK);

        response.assert_text(1196282.to_string());
    }
}
