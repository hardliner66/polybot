#[macro_use]
extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn root() -> String {
    "It seems you are too early. Maybe come back at a latet time!".to_string()
}

#[launch]
fn rocket() -> rocket::Rocket {
    dotenv::dotenv().ok();
    rocket::ignite()
        .mount("/", routes![root])
        .mount("/hello", routes![hello])
}
