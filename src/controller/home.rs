use std::collections::HashMap;

use rocket_dyn_templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("tera/home/index", context)
}
