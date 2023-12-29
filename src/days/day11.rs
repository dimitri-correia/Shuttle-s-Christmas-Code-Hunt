use std::io::Cursor;

use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use image::{io::Reader as ImageReader, GenericImageView, Rgba};
use tower_http::services::ServeDir;

pub fn get_day_11_router() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/red_pixels", post(red_pixels))
}

async fn red_pixels(mut multipart: Multipart) -> (StatusCode, String) {
    let mut res = 0;
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() != "image" {
            continue;
        }
        let data = field.bytes().await.unwrap();

        let img = ImageReader::new(Cursor::new(&data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        res = img.pixels().filter(is_magical_red()).count();
    }

    (StatusCode::OK, res.to_string())
}

fn is_magical_red() -> fn(&(u32, u32, Rgba<u8>)) -> bool {
    |(_x, _y, Rgba([r, g, b, _a]))| *r as u16 > (*g as u16 + *b as u16)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_11_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/assets/decoration.png").await;

        response.assert_status(StatusCode::OK);

        assert!(response
            .headers()
            .get("content-type")
            .is_some_and(|v| v == "image/png"));
        assert!(response
            .headers()
            .get("content-length")
            .is_some_and(|v| v == "787297"));

        assert_eq!(
            response.as_bytes(),
            include_bytes!("../../assets/decoration.png") as &[u8]
        );
    }

    // TODO not yet possible see https://github.com/JosephLenton/axum-test/issues/51
    // #[tokio::test]
    // async fn task2() {
    //     let app = get_day_11_router();
    //
    //     // Run the application for testing.
    //     let server = TestServer::new(app).unwrap();
    //
    //     // Send the request.
    //     let form = Form::new().part(
    //         "image",
    //         Part::bytes(include_bytes!("../../assets/decoration.png").as_slice())
    //             .file_name("decoration.png")
    //             .mime_str("image/png")
    //             .unwrap(),
    //     );
    //     let response = server.post("/red_pixels").multipart(form).await;
    //
    //     response.assert_status(StatusCode::OK);
    //
    //     //73034
    // }
}
