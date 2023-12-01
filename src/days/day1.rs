use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tracing::info;

pub fn get_day_1_router() -> Router {
    Router::new().route("/*l_nums", get(cube_the_bits))
}

async fn cube_the_bits(Path(l_nums): Path<String>) -> (StatusCode, String) {
    let l_nums = l_nums.split("/").map(|n| n.parse::<i32>().unwrap());
    // Perform XOR operation
    let mut xor_result = 0;
    for num in l_nums {
        xor_result ^= num;
    }

    // Raise the result to the power of 3
    let result_pow_3 = xor_result.pow(3);

    info!(result_pow_3);

    (StatusCode::OK, format!("{result_pow_3}"))
}
