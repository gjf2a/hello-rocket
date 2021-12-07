// From https://rocket.rs/v0.5-rc/guide/getting-started/
// Modified by me to reference static files, experiment, etc...

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

// Template guide: https://api.rocket.rs/master/rocket_dyn_templates/index.html

#[get("/form_submitted?<user_name>")]
fn named(user_name: &str) -> Template {
    let mut context = HashMap::new();
    context.insert("user_name", user_name.to_owned());
    Template::render("welcome", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![named])
        .mount("/", FileServer::from("static/"))
}