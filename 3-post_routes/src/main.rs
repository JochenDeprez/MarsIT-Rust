#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
struct LoginInfo {
    email: String,
    password: String,
}

#[post("/user/login", data="<login_info>", format="json")]
fn login_user(login_info: Json<LoginInfo>) -> String {
    format!("Email:{}, password: {}", login_info.email, login_info.password)
}

fn main() {
    rocket::ignite().mount("/api", routes![login_user]).launch();
}