#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> rocket::Rocket {
    dotenv::dotenv().ok();
    rocket::ignite().mount("/hello", routes![hello])
}
