#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

#[get("/")]
fn hello() ->  JsonValue {
    json!("hello world")
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount("/", routes![hello])
        .launch()
        .await;
}
