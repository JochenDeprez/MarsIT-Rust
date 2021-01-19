#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/user/<name>")]
fn url_parameter(name: String) -> String {
    format!("Welcome to the application {}", name)
}

#[get("/user?<name>&<salutation>")]
fn url_query_parameter(name: String, salutation: Option<String>) -> String {
    match salutation {
        Some(s) => format!("{} {}", s, name),
        None => format!("Welcome {}", name)
    }
}

fn main() {
    rocket::ignite().mount("/api", routes![url_parameter, url_query_parameter]).launch();
}