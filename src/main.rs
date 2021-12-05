// From https://rocket.rs/v0.5-rc/guide/getting-started/
// Modified by me to reference static files, experiment, etc...

#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

#[get("/form_submitted?<user_name>")]
fn named(user_name: &str) -> String {
    format!("Welcome {:?}!", user_name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![named])
        .mount("/", FileServer::from("static/"))
}