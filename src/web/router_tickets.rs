use axum::extract::State;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::model::{ModelController, Ticket, TicketForCreate};

use crate::{Error, Result};

struct Params {
    name: Option<String>,
}
use axum::extract::{Path, Query};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/ticket", post(create_ticket))
        .route("/tickets", get(list_tickets))
        .route("/ticket/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(model): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Value>> {
    println!("--> {:<12} create_ticket", "INFO");
    let ticket = model.new_ticket(ticket_fc).await?;
    Ok(Json(json!(ticket)))
}

async fn list_tickets(State(model): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("--> {:<12} list_tickets", "INFO");
    let tickets = model.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(model): State<ModelController>,
    Path(id): Path<u32>,
) -> Result<Json<Ticket>> {
    println!("--> {:<12} delete_ticket: id={}", "INFO", id);
    let ticket = model.delete_ticket(id).await?;
    Ok(Json(ticket))
}
