#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::LenientForm;

#[get("/")]
fn index() -> &'static str {
    "Pair Matching Bot"
}

#[derive(FromForm,Debug)]
struct NewMatchingRequest {
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: String,
    response_url: String,
}

#[post("/new-matching", data = "<new_matching_request>")]
fn new_matching(new_matching_request: LenientForm<NewMatchingRequest>) -> String {
    format!("Received: {:?}", new_matching_request)
}

fn main() {
    rocket::ignite().mount("/", routes![
        index,
        new_matching
    ]).launch();
}
