use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub fn get_day_15_router() -> Router {
    Router::new()
        .route("/nice", post(nice))
        .route("/game", post(game))
}

const NICE: &str = "nice";
const NAUGHTY: &str = "naughty";

#[derive(Serialize, Deserialize)]
struct Input {
    input: String,
}

#[derive(Serialize, Deserialize)]
struct Result {
    result: String,
}

async fn nice(Json(input): Json<Input>) -> (StatusCode, Json<Result>) {
    if is_nice(&input.input) {
        (
            StatusCode::OK,
            Json(Result {
                result: NICE.to_string(),
            }),
        )
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(Result {
                result: NAUGHTY.to_string(),
            }),
        )
    }
}

fn is_nice(input: &str) -> bool {
    const VOWELS: &str = "aeiouy";
    const BAD_SUBSTRS: [&[u8]; 4] = [&[b'a', b'b'], &[b'c', b'd'], &[b'p', b'q'], &[b'x', b'y']];

    // Must contain at least three vowels (aeiouy),
    // at least one letter that appears twice in a row,
    // and must not contain the substrings: ab, cd, pq, or xy.

    let at_least_3_vowels = input.chars().filter(|c| VOWELS.contains(*c)).count() >= 3;

    let letter_twice_in_a_row = input
        .as_bytes()
        .windows(2)
        .any(|b| b[0].is_ascii_alphabetic() && b[0] == b[1]);

    let contains_bad_substring = input
        .as_bytes()
        .windows(2)
        .any(|b| BAD_SUBSTRS.contains(&b));

    at_least_3_vowels && letter_twice_in_a_row && !contains_bad_substring
}

#[derive(Serialize, Deserialize)]
struct ResultWithReason {
    result: String,
    reason: String,
}

async fn game(Json(input): Json<Input>) -> (StatusCode, Json<ResultWithReason>) {
    const RULE_BREAK: [(StatusCode, &str); 10] = [
        (StatusCode::BAD_REQUEST, "8 chars"),
        (StatusCode::BAD_REQUEST, "more types of chars"),
        (StatusCode::BAD_REQUEST, "55555"),
        (StatusCode::BAD_REQUEST, "math is hard"),
        (StatusCode::NOT_ACCEPTABLE, "not joyful enough"), //406
        (
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "illegal: no sandwich",
        ), //451
        (StatusCode::RANGE_NOT_SATISFIABLE, "outranged"),  //416
        (StatusCode::UPGRADE_REQUIRED, "😳"),              //426
        (StatusCode::IM_A_TEAPOT, "not a coffee brewer"),  //418
        (StatusCode::OK, "that's a nice password"),
    ];

    let rule_break = get_rule_break(input.input);

    if let Some(rule_break) = rule_break {
        (
            RULE_BREAK[rule_break].0,
            Json(ResultWithReason {
                result: NAUGHTY.to_string(),
                reason: RULE_BREAK[rule_break].1.to_string(),
            }),
        )
    } else {
        (
            RULE_BREAK[9].0,
            Json(ResultWithReason {
                result: NICE.to_string(),
                reason: RULE_BREAK[9].1.to_string(),
            }),
        )
    }
}

fn get_rule_break(input: String) -> Option<usize> {
    if input.len() < 8 {
        return Some(0);
    }
    let uppercase = input.chars().find(|c| c.is_uppercase());
    let lowercase = input.chars().find(|c| c.is_lowercase());
    let digit = input.chars().find(char::is_ascii_digit);
    if uppercase.is_none() || lowercase.is_none() || digit.is_none() {
        return Some(1);
    }
    if input.chars().filter(char::is_ascii_digit).count() < 5 {
        return Some(2);
    }
    if check_sum_of_integers_to_be_2023(&input) {
        return Some(3);
    }
    // Rule 5: must contain the letters j, o, and y in that order and in no other order
    // Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
    // Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
    // Rule 8: must contain at least one emoji
    // Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an a
    None
}

fn check_sum_of_integers_to_be_2023(input: &str) -> bool {
    let (a, b) = input.chars().fold((0, 0), |(current_integer, sum), c| {
        if c.is_ascii_digit() {
            (current_integer * 10 + c.to_digit(10).unwrap() as i32, sum)
        } else {
            (0, sum + current_integer)
        }
    });
    2023 == a + b
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn task1a() {
        let app = get_day_15_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/nice")
            .json(&json!({"input": "hello there"}))
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({"result":"nice"}));
    }

    #[tokio::test]
    async fn task1b() {
        let app = get_day_15_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.post("/nice").json(&json!({"input": "abcd"})).await;

        response.assert_status(StatusCode::BAD_REQUEST);

        response.assert_json(&json!({"result":"naughty"}));
    }

    #[test]
    fn test_valid_input() {
        assert!(check_sum_of_integers_to_be_2023("abc123xyz45pqr678stu1177"));
        assert!(check_sum_of_integers_to_be_2023("abc2000def0023"));
        assert!(check_sum_of_integers_to_be_2023("abc2023def"));
        assert!(check_sum_of_integers_to_be_2023("2023"));
    }

    #[test]
    fn test_invalid_input() {
        assert!(!check_sum_of_integers_to_be_2023("abc"));
        assert!(!check_sum_of_integers_to_be_2023("2022"));
        assert!(!check_sum_of_integers_to_be_2023("a2b0c2d3e"));
    }
}
