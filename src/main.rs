#![allow(dead_code)]

#[macro_use]
extern crate rocket;

mod controller;
mod domain;
mod utils;

use crate::utils::database::Db;
use controller::{auth, home, login};
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
        .mount("/login", routes![login::index])
        .mount("/auth", routes![auth::google_auth])
        .mount("/auth", routes![auth::logout])
        .mount("/static", FileServer::from(static_dir))
        .attach(Template::fairing())
        .attach(Db::fairing())
}
