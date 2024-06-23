use crate::{Error, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::{Json, Router};
use axum::routing::{post, Route};
use tower_cookies::{Cookie, Cookies};
use crate::web;
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))

}

async fn login(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("--> {:<12} - handler_login", "HANDLER");
    if payload.username == "user" && payload.password == "123" {

        cookie.add( Cookie::new(web::AUTHEN_TOKEN, "authen-info"));
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

