#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::{ RawStr, uri::Uri };
use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};
use voca_rs::case;
use std::collections::HashMap;

#[get("/")]
fn root() -> Template {
    let mut context = HashMap::new();
    context.insert("main", "Kut man. Echt jammer.");
    Template::render("index", &context)
}

#[get("/<ding>")]
fn ding(ding: &RawStr) -> Template {
    let decoded = Uri::percent_decode(ding.as_bytes()).unwrap_or_default();
    let capitalized = case::capitalize(&decoded, true);
    let mut context = HashMap::new();
    context.insert("main", format!("{}... Mooi kut.", capitalized));
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            root,
            ding,
        ])
        .mount("/public", StaticFiles::from("./public"))
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;

    #[test]
    fn ding() {
        let client = Client::new(rocket()).expect("is rocket");
        let mut response = client.get("/regen").dispatch();
        assert_eq!(response.body_string(), Some("Regen... mooi kut man".into()));

    }
}
