use axum::extract::State;
use axum::Json;

use crate::model::{Ticket, ModelController, TicketForCreate};

use crate::{Error, Result};


async fn create_ticket(
    State(model): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    let ticket = model.new_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}


async fn list_tickets(
    State(model): State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
    let tickets = model.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(model): State<ModelController>,
    Json(id): Json<u32>
) -> Result<Json<Ticket>> {
    let ticket = model.delete_ticket(id).await?;
    Ok(Json(ticket))
}
