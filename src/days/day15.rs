use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
    const BAD_SUBSTRINGS: [&[u8]; 4] = [&[b'a', b'b'], &[b'c', b'd'], &[b'p', b'q'], &[b'x', b'y']];

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
        .any(|b| BAD_SUBSTRINGS.contains(&b));

    at_least_3_vowels && letter_twice_in_a_row && !contains_bad_substring
}

#[derive(Serialize, Deserialize)]
struct ResultWithReason {
    result: String,
    reason: String,
}

#[derive(PartialEq, Eq)]
enum RuleViolation {
    Length,
    UppercaseLowercaseDigit,
    DigitsCount,
    SumOfIntegers,
    JOYOrder,
    MirrorLetters,
    UnicodeCharacter,
    Emoji,
    HashEndingWithA,
    None,
}

impl RuleViolation {
    fn message(&self) -> (StatusCode, &str) {
        match self {
            RuleViolation::Length => (StatusCode::BAD_REQUEST, "8 chars"),
            RuleViolation::UppercaseLowercaseDigit => {
                (StatusCode::BAD_REQUEST, "more types of chars")
            }
            RuleViolation::DigitsCount => (StatusCode::BAD_REQUEST, "55555"),
            RuleViolation::SumOfIntegers => (StatusCode::BAD_REQUEST, "math is hard"),
            RuleViolation::JOYOrder => (StatusCode::NOT_ACCEPTABLE, "not joyful enough"), //406
            RuleViolation::MirrorLetters => (
                StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                "illegal: no sandwich",
            ), //451
            RuleViolation::UnicodeCharacter => (StatusCode::RANGE_NOT_SATISFIABLE, "outranged"), //416
            RuleViolation::Emoji => (StatusCode::UPGRADE_REQUIRED, "😳"), //426
            RuleViolation::HashEndingWithA => (StatusCode::IM_A_TEAPOT, "not a coffee brewer"), //418
            _ => (StatusCode::OK, "that's a nice password"),
        }
    }
}

async fn game(Json(input): Json<Input>) -> (StatusCode, Json<ResultWithReason>) {
    let rule_break = get_rule_break(input.input);
    let rule_break_message = rule_break.message();

    (
        rule_break_message.0,
        Json(ResultWithReason {
            result: if rule_break == RuleViolation::None {
                NICE.to_string()
            } else {
                NAUGHTY.to_string()
            },
            reason: rule_break_message.1.to_string(),
        }),
    )
}

fn get_rule_break(input: String) -> RuleViolation {
    // Rule 1: must be at least 8 characters long
    if input.len() < 8 {
        return RuleViolation::Length;
    }
    // Rule 2: must contain uppercase letters, lowercase letters, and digits
    if !contain_upper_lower_digit(&input) {
        return RuleViolation::UppercaseLowercaseDigit;
    }
    // Rule 3: must contain at least 5 digits
    if input.chars().filter(char::is_ascii_digit).count() < 5 {
        return RuleViolation::DigitsCount;
    }
    // Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
    if !check_sum_of_integers_to_be_2023(&input) {
        return RuleViolation::SumOfIntegers;
    }
    // Rule 5: must contain the letters j, o, and y in that order and in no other order
    if !contains_in_order(&input, ['j', 'o', 'y']) {
        return RuleViolation::JOYOrder;
    }
    // Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
    if !contains_mirror(&input) {
        return RuleViolation::MirrorLetters;
    }
    // Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
    if !contains_char_in_range(&input, '\u{2980}', '\u{2BFF}') {
        return RuleViolation::UnicodeCharacter;
    }
    // Rule 8: must contain at least one emoji
    if emojito::find_emoji(&input).is_empty() {
        return RuleViolation::Emoji;
    }
    // Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an 'a'
    if !sha256_ends_with_a(&input) {
        return RuleViolation::HashEndingWithA;
    }
    RuleViolation::None
}

fn sha256_ends_with_a(input: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result).ends_with('a')
}

fn contains_char_in_range(input: &str, start: char, end: char) -> bool {
    input.chars().any(|c| (start..=end).contains(&c))
}

fn contains_mirror(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|str| str[0].is_ascii_alphabetic() && str[1].is_ascii_alphabetic() && str[0] == str[2])
}

fn contains_in_order(input: &str, letters: [char; 3]) -> bool {
    if letters
        .iter()
        .any(|expected_char| input.chars().filter(|c| c == expected_char).count() > 1)
    {
        return false;
    }

    let mut chars = input.chars(); // will be consumed char by char
    letters
        .iter()
        .all(|&expected_char| chars.any(|c| c == expected_char))
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

fn contain_upper_lower_digit(input: &str) -> bool {
    let uppercase = input.chars().find(|c| c.is_uppercase());
    let lowercase = input.chars().find(|c| c.is_lowercase());
    let digit = input.chars().find(char::is_ascii_digit);
    uppercase.is_some() && lowercase.is_some() && digit.is_some()
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
    fn test_check_sum_of_integers_to_be_2023() {
        assert!(check_sum_of_integers_to_be_2023("abc123xyz45pqr678stu1177"));
        assert!(check_sum_of_integers_to_be_2023("abc2000def0023"));
        assert!(check_sum_of_integers_to_be_2023("abc2023def"));
        assert!(check_sum_of_integers_to_be_2023("2023"));
        assert!(!check_sum_of_integers_to_be_2023("abc"));
        assert!(!check_sum_of_integers_to_be_2023("2022"));
        assert!(!check_sum_of_integers_to_be_2023("a2b0c2d3e"));
    }

    #[test]
    fn test_contains_j_o_y_in_order() {
        fn contains_j_o_y_in_order(s: &str) -> bool {
            contains_in_order(s, ['j', 'o', 'y'])
        }

        assert!(contains_j_o_y_in_order("joy"));
        assert!(contains_j_o_y_in_order("0j1o2y3"));
        assert!(contains_j_o_y_in_order("0joy1"));
        assert!(!contains_j_o_y_in_order("yoj"));
        assert!(!contains_j_o_y_in_order("jo"));
        assert!(!contains_j_o_y_in_order("oy"));
        assert!(!contains_j_o_y_in_order("oyj"));
        assert!(!contains_j_o_y_in_order("joyjoy"));
    }

    #[test]
    fn test_contains_mirror() {
        assert!(contains_mirror("xyx"));
        assert!(contains_mirror("qqxyxee"));
        assert!(!contains_mirror("diid"));
        assert!(!contains_mirror("dim"));
        assert!(!contains_mirror("121"));
        assert!(!contains_mirror("q2q"));
    }

    #[tokio::test]
    async fn task2a() {
        let app = get_day_15_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/game")
            .json(&json!({"input": "password"}))
            .await;

        response.assert_status(StatusCode::BAD_REQUEST);

        response.assert_json(&json!({"result":"naughty","reason":"more types of chars"}));
    }

    #[tokio::test]
    async fn task2b() {
        let app = get_day_15_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/game")
            .json(&json!({"input": "Password12345"}))
            .await;

        response.assert_status(StatusCode::BAD_REQUEST);

        response.assert_json(&json!({"result":"naughty","reason":"math is hard"}));
    }

    #[tokio::test]
    async fn task2c() {
        let app = get_day_15_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/game")
            .json(&json!({"input": "23jPassword2000y"}))
            .await;

        response.assert_status(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS);

        response.assert_json(&json!({"result":"naughty","reason":"illegal: no sandwich"}));
    }
}
