#![allow(dead_code)]

#[macro_use]
extern crate rocket;

mod controller;
mod utils;

use controller::*;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use utils::{env, googleapi};

#[launch]
fn rocket() -> _ {
    let static_dir = env::get_or_else(
        String::from("STATIC_DIR"),
        String::from(relative!("static")),
    );

    let google_api_secrets = googleapi::load();
    rocket::build()
        .manage(google_api_secrets)
        .mount("/", routes![home::index])
        .mount("/static", FileServer::from(static_dir))
        .attach(Template::fairing())
}
