#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "hello world"
}

#[rocker::main]
async fn main() {
    let _ = rocket::ignite()
        .mount("/", routes![hello])
        .launch()
        .await;
}
