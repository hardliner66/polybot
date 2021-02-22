#[macro_use]
extern crate rocket;

use rocket::{http::Status, outcome::Outcome, request::FromRequest};

struct ApiKey(String);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
    BadCount,
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    std::fs::read_to_string("./apikeys.txt")
        .unwrap_or_default()
        .lines()
        .any(|line| key == line)
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        if std::env::args().any(|x| x == "--dev") {
            return Outcome::Success(ApiKey("".to_string()));
        }
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

#[get("/<name>/<age>")]
fn hello(_key: ApiKey, name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn root() -> String {
    "It seems you are too early. Maybe come back at a later time!".to_string()
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::ignite()
        .mount("/", routes![root])
        .mount("/hello", routes![hello])
}
