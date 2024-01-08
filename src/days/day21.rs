use std::str::FromStr;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use iso_country::Country;
use reqwest::Client;
use s2::cell::Cell;
use s2::cellid::CellID;

use crate::days::day21::LatLong::{Lat, Long};

pub fn get_day_21_router() -> Router {
    let client = Client::new();
    Router::new()
        .route("/coords/:binary", get(coords))
        .route("/country/:binary", get(country))
        .with_state(client)
}

#[derive(Debug)]
enum Cardinal {
    N,
    S,
    E,
    W,
}

enum LatLong {
    Lat,
    Long,
}

async fn coords(Path(binary): Path<String>) -> (StatusCode, String) {
    let center = Cell::from(CellID(u64::from_str_radix(&binary, 2).unwrap())).center();
    let lat = from_deg(center.latitude().deg(), Lat);
    let lon = from_deg(center.longitude().deg(), Long);

    (StatusCode::OK, format!("{} {}", lat, lon))
}

fn from_deg(angle: f64, lat_long: LatLong) -> String {
    let degrees = angle.abs().floor();
    let minutes = ((angle.abs() - degrees) * 60.0).floor();
    let seconds = (angle.abs() - degrees - minutes / 60.0_f64) * 3600.0_f64;
    let cardinal = match lat_long {
        Lat => {
            if angle < 0.0 {
                Cardinal::S
            } else {
                Cardinal::N
            }
        }
        Long => {
            if angle < 0.0 {
                Cardinal::W
            } else {
                Cardinal::E
            }
        }
    };
    format!(
        "{}°{}'{:.3}''{:?}",
        (degrees as u16) % 180,
        minutes as u8,
        seconds,
        cardinal
    )
}

async fn country(Path(binary): Path<String>, client: Client) -> (StatusCode, String) {
    let center = Cell::from(CellID(u64::from_str_radix(&binary, 2).unwrap())).center();

    (
        StatusCode::OK,
        get_country_from_coordinates(center.latitude().deg(), center.longitude().deg(), client)
            .await,
    )
}

async fn get_country_from_coordinates(lat: f64, lon: f64, client: Client) -> String {
    let url = format!("https://nominatim.openstreetmap.org/reverse?lat={lat}&lon={lon}");

    let response = client.get(&url).send().await.unwrap().text().await.unwrap();

    let regex = regex::Regex::new(r".*<country_code>(.+)</country_code>.*").unwrap();

    let (_, [country_code]): (&str, [&str; 1]) = regex.captures(&response).unwrap().extract();

    let country_code = country_code.to_ascii_uppercase();
    let a = Country::from_str(&country_code)
        .unwrap()
        .name()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();
    dbg!(&a);
    a
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1a() {
        let app = get_day_21_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/coords/0100111110010011000110011001010101011111000010100011110001011011")
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text("83°39'54.324''N 30°37'40.584''W");
    }

    #[tokio::test]
    async fn task1b() {
        let app = get_day_21_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/coords/0010000111110000011111100000111010111100000100111101111011000101")
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text("18°54'55.944''S 47°31'17.976''E");
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_21_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/country/0010000111110000011111100000111010111100000100111101111011000101")
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text("Madagascar");
    }
}
