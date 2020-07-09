#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use voca_rs::*;

#[get("/")]
fn root() -> &'static str {
    "Kut man, echt jammer."
}

#[get("/<ding>")]
fn ding(ding: &RawStr) -> String {
    let capitalized = case::capitalize(ding.as_str(), true);
    format!("{}... mooi kut man", capitalized)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        root,
        ding,
    ])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn root() {
        let client = Client::new(rocket()).expect("is rocket");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("Kut man, echt jammer.".into()));
    }

    #[test]
    fn ding() {
        let client = Client::new(rocket()).expect("is rocket");
        let mut response = client.get("/regen").dispatch();
        assert_eq!(response.body_string(), Some("Regen... mooi kut man".into()));
    }
}
