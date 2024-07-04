use std::net::SocketAddr;

use axum::extract::{Query, Path};
use axum::response::Response;
use axum::response::{Html, IntoResponse};
use axum::{middleware, Json, Router};
use axum::routing::{get, get_service, Route};
use axum::handler;
use model::ModelController;
use serde::Deserialize;
use tower_cookies::{Cookie, CookieManager, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use std::fmt::{format, Debug};
pub use self::errors::{Error, Result};  
mod errors;
mod web;
mod model;
mod ctx;
struct  SumationError;
#[derive(Debug, Deserialize)]
struct Params{
    name: Option<String>
}

impl  Debug for SumationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SumationError!!!").finish()
    }
}

#[tokio::main]
async fn main() {
    let mc = ModelController::new().await.unwrap();
    let app = Router::new().merge(router_hello());
    let api_app = web::router_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_authen::middleware_authen));
    let all_app = app
    .merge(web::router_login::routes())
    .nest("/api", api_app)
    .layer(middleware::map_response(main_respone_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(router_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(all_app.into_make_service())
        .await
        .unwrap();
}

fn router_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}


async fn main_respone_mapper(res: Response) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");
    res
}

fn router_hello() -> Router {
    Router::new()
        .route("/hello",get(|| async { "<h1>Hello, World! here is rust server</h1>" }))
        .route("/getstr", get(get_assets))
        .route("/getname/:name", get(get_user))
        
}

async fn get_assets(cookies: Cookies, Query(param): Query<Params>) -> impl IntoResponse {
    cookies.add(Cookie::new("hello", "world"));
    let name = param.name.as_deref().unwrap_or("world");    
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Json(format!("hello {name}"))
}

async fn get_user(Path(params): Path<Params>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world");
    Json(format!("name: {}", name))
}
