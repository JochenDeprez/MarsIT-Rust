#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Deserialize;

use rocket_contrib::json::{Json, JsonValue};

use rocket::http::{Status, ContentType};
use rocket::response::Responder;
use rocket::{Response, Request, response};

#[derive(Deserialize)]
struct LoginInfo {
    email: String,
    password: String,
}

struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[post("/user/login", data="<login_info>", format="json")]
fn login_user(login_info: Json<LoginInfo>) -> ApiResponse {
    ApiResponse{
        json: rocket_contrib::json!({"msg": format!("Email:{}, password: {}", login_info.email, login_info.password)}),
        status: Status::Ok
    }
}

fn main() {
    rocket::ignite().mount("/api", routes![login_user]).launch();
}