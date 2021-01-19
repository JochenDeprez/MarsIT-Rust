#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::{Status, ContentType};
use rocket::request::{FromRequest};
use rocket::response::{Responder};
use rocket::{Response, Request, response, request, Outcome};

use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};

use jsonwebtoken::{encode, decode, Validation, EncodingKey, DecodingKey};

use rocket::http::{RawStr, Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status
}

#[derive(Deserialize)]
struct LoginInfo {
    email: String,
    password: String,
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

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

fn jwt(email: String) -> jsonwebtoken::errors::Result<String> {
    let my_claims = Claims {exp: 10000000000 , email: email};
    encode(&jsonwebtoken::Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, people of MarsIT!"
}

#[get("/user/<name>")]
fn url_parameter(name: String) -> ApiResponse{
    ApiResponse{
        json: rocket_contrib::json!({"msg": format!("Welcome {}", name)}),
        status: Status::Ok
    }
}

#[get("/user?<name>&<salutation>")]
fn url_query_parameter(name: String, salutation: Option<String>) -> ApiResponse{
    let mut sentence;
    match salutation {
        Some(s) => sentence = format!("{} {}", s, name),
        None => sentence = format!{"Welcome {}", name}
    }
    ApiResponse{
        json: rocket_contrib::json!({"msg": sentence}),
        status: Status::Ok
    }
}

#[post("/user/login", data="<login_info>", format="json")]
fn login_user(login_info: Json<LoginInfo>) -> ApiResponse {
    //check if user exists in database
    let user_exists = true;
    let jwt_token = jwt(login_info.email.to_string()).unwrap();

    if user_exists {
        ApiResponse{
            json: rocket_contrib::json!({"msg": format!("user {} logged in", login_info.email),
                                        "jwt": jwt_token}),
            status: Status::Ok
        }
    } else {
        ApiResponse{
            json: rocket_contrib::json!({"msg": format!("user {} was not logged in", login_info.email)}),
            status: Status::Unauthorized
        }
    }
}

#[get("/user/supersecret")]
fn user_secret(token:Token) -> ApiResponse{
    ApiResponse{
        json: rocket_contrib::json!({"msg": "You have accessed the super secret data"}),
        status: Status::Ok
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::All;
    let cors = rocket_cors::CorsOptions{
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }.to_cors()?;
    rocket::ignite().mount("/api", routes![index, url_parameter, url_query_parameter, login_user, user_secret])
        .attach(cors)
        .launch();

    Ok(())
}
