#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::{Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;

#[get("/")]
fn index() -> &'static str {
    "Hello people of MarsIT!"
}

fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::All;
    let cors = rocket_cors::CorsOptions{
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;
    rocket::ignite().mount("/api", routes![index])
        .attach(cors)
        .launch();

    Ok(())
}