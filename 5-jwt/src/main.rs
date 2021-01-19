#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::{Json, JsonValue};
use rocket::http::{Status, ContentType};
use rocket::response::Responder;

use rocket::{Response, Request, response, request, Outcome};
use serde::{Deserialize, Serialize};
use rocket::request::{FromRequest};
use jsonwebtoken::{encode, decode, Validation, EncodingKey, DecodingKey};

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

#[derive(Deserialize, Serialize)]
struct Claims {
    exp: usize,
    email: String,
}

struct Token(String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let exists = request.headers().contains(&*"auth".to_string());
        if exists{
            let token = request.headers().get_one("auth").unwrap().to_string();
            let decoded = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).unwrap().claims;
            if decoded.email == "jochen.deprez@student.howest.be" {
                Outcome::Success(Token(token.to_string()))
            } else {
                Outcome::Failure((Status::Unauthorized, "error".to_string()))
            }
        } else {
            Outcome::Failure((Status::Unauthorized, "No auth in header".to_string()))
        }
    }
}

fn jwt(email: String) -> jsonwebtoken::errors::Result<String> {
    let my_claims = Claims {exp: 10000000000 , email: email};
    encode(&jsonwebtoken::Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
}

#[post("/user/login", data="<login_info>", format="json")]
fn login_user(login_info: Json<LoginInfo>) -> ApiResponse {
    //check if combination exists in DB
    let exists = true;
    if exists {
        let jwt_token = jwt(login_info.email.to_string()).unwrap();

        ApiResponse{
            json: rocket_contrib::json!({"msg": "User logged in",
                                         "jwt": jwt_token}),
            status: Status::Ok
        }
    } else {
        ApiResponse{
            json: rocket_contrib::json!({"msg": "User or combination does not exist"}),
            status: Status::Unauthorized
        }
    }
}

#[get("/data/supersecret")]
fn get_supersecret_data(token: Token) -> ApiResponse {
    let decoded = decode::<Claims>(&token.0, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).unwrap().claims;
    let email = decoded.email;
    ApiResponse{
        json: rocket_contrib::json!({"msg": "you can access the super secret data: foo",
                                             "email": email}),
        status: Status::Ok
    }
}

fn main() {
    rocket::ignite().mount("/api", routes![login_user, get_supersecret_data]).launch();
}