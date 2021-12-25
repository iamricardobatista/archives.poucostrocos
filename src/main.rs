#![allow(dead_code)]

#[macro_use]
extern crate rocket;

mod controller;
mod domain;
mod utils;

use controller::home;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use utils::env;

#[launch]
fn rocket() -> _ {
    let static_dir = env::get_or_else(
        String::from("STATIC_DIR"),
        String::from(relative!("static")),
    );

    rocket::build()
        .mount("/", routes![home::index])
        .mount("/static", FileServer::from(static_dir))
        .attach(Template::fairing())
}
