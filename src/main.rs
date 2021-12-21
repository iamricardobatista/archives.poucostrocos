#![allow(dead_code)]
#[macro_use]
extern crate rocket;

mod domain;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing())
}
