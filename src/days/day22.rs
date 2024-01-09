use std::collections::HashMap;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use pathfinding::prelude::bfs;

pub fn get_day_22_router() -> Router {
    Router::new()
        .route("/integers", post(integers))
        .route("/rocket", post(rocket))
}

async fn integers(payload: String) -> (StatusCode, String) {
    let num = usize::try_from(
        payload
            .lines()
            .map(|el| el.trim().parse::<u64>().unwrap())
            .fold(0u64, |acc, el| acc ^ el),
    )
    .unwrap();

    (StatusCode::OK, "🎁".repeat(num).to_string())
}

#[derive(Copy, Clone)]
struct Star {
    x: i32,
    y: i32,
    z: i32,
}

async fn rocket(payload: String) -> impl IntoResponse {
    let (stars, portals): (Vec<Star>, HashMap<i32, Vec<i32>>) = parse_payload(payload);

    let path = bfs(
        &0,
        |node| portals[&node].clone(),
        |&node| node == (stars.len() as i32 - 1),
    )
    .unwrap();

    let path_without_portal: f32 = path
        .windows(2)
        .map(|star| path_without_portal(stars[star[0usize] as usize], stars[star[1usize] as usize]))
        .sum();

    (
        StatusCode::OK,
        format!("{} {:.3}", path.len() - 1, path_without_portal),
    )
}

fn path_without_portal(star1: Star, star2: Star) -> f32 {
    let dx = (star1.x - star2.x) as f32;
    let dy = (star1.y - star2.y) as f32;
    let dz = (star1.z - star2.z) as f32;

    let distance_squared = dx * dx + dy * dy + dz * dz;

    distance_squared.sqrt()
}

fn parse_payload(payload: String) -> (Vec<Star>, HashMap<i32, Vec<i32>>) {
    let mut lines = payload.lines();
    let n = lines.next().unwrap().trim().parse::<u32>().unwrap();

    let stars = (0..n)
        .map(|_| {
            let star = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let (x, y, z) = match star.len() {
                3 => (star[0], star[1], star[2]),
                _ => panic!("Invalid coordinates for star"),
            };
            Star { x, y, z }
        })
        .collect::<Vec<Star>>();

    let k = lines.next().unwrap().trim().parse::<u32>().unwrap();

    let mut portals: HashMap<i32, Vec<i32>> = HashMap::new();

    (0..k).for_each(|_| {
        let portal = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (star_a_id, star_b_id) = match portal.len() {
            2 => (portal[0], portal[1]),
            _ => panic!("Invalid portal"),
        };
        portals.entry(star_a_id).or_default().push(star_b_id);
        portals.entry(star_b_id).or_default().push(star_a_id);
    });

    (stars, portals)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let app = get_day_22_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/integers")
            .text(
                "888
77
888
22
77",
            )
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text("🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁");
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_22_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/rocket")
            .text(
                "5
0 1 0
-2 2 3
3 -3 -5
1 1 5
4 3 5
4
0 1
2 4
3 4
1 2",
            )
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_text("3 26.123");
    }
}
