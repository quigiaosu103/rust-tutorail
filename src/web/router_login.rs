use crate::web;
use crate::{Error, Result};
use axum::routing::{post, Route};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}

async fn login(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("--> {:<12} - handler_login", "HANDLER");
    if payload.username == "user" && payload.password == "123" {
        cookie.add(Cookie::new(
            web::AUTHEN_TOKEN,
            "user-1.exp.sign",
        ));
        let body = Json(json!({
            "result": {
                "status": "ok",
            }
        }));
        Ok(body)
    } else {
        Err(Error::LoginFailed)
    }
}
