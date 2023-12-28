use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tracing::info;

pub fn get_day_1_router() -> Router {
    Router::new().route("/*l_nums", get(cube_the_bits))
}

async fn cube_the_bits(Path(l_nums): Path<String>) -> (StatusCode, String) {
    let res = l_nums
        .split('/')
        .map(|n| n.parse::<i32>().unwrap())
        .fold(0, |acc, v| acc ^ v)
        .pow(3);

    info!(res);

    (StatusCode::OK, format!("{res}"))
}
