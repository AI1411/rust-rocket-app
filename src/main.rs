#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;
use rocket::response::status;

#[get("/")]
fn hello() -> JsonValue {
    json!("hello world")
}

#[get("/rustaceans")]
fn get_rustaceans() -> JsonValue {
    json!([{"id": 1, "name": "Akira Ishii", "email": "akira@gmail.com"}, {"id": 2, "name": "sakura tomita", "email": "tomita@gmail.com"}])
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> JsonValue {
    json!({"id": id, "name": "Akira Ishii", "email": "akira@gmail.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustaceans() -> JsonValue {
    json!({"id": 3, "name": "sakura", "email": "test@gmail.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id: i32) -> JsonValue {
    json!({"id": id, "name": "akira ishii", "email": "ishii@gmail.com"})
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!("Not Found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            create_rustaceans,
            update_rustaceans,
            delete_rustacean,
        ]).register(catchers![
            not_found
        ])
        .launch()
        .await;
}
