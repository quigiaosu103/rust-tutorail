use crate::{Error, Result};
use axum::routing::delete;
use serde::{Deserialize, Serialize};
use std::{clone, sync::{Arc, Mutex}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ticket {
    id: u32,
    title: String
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct  TicketForCreate {
    title: String
}

#[derive(Clone)]
pub struct ModelController {
    store: Arc<Mutex<Vec<Option<Ticket>>>>
}
 

// constructor for ModelController
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self{
            store: Arc::default()
        })
    }
}


// implement CURD for ModelController
impl ModelController {
    pub async fn new_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store: std::sync::MutexGuard<Vec<Option<Ticket>>>  = self.store.lock().unwrap();
        let id = store.len() as u32;
        let ticket = Ticket{
            id,
            title: ticket_fc.title
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }


    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store  = self.store.lock().unwrap();
        let tickets =store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u32) -> Result<Ticket> {
        let mut store: std::sync::MutexGuard<Vec<Option<Ticket>>> = self.store.lock().unwrap();
        let mut  ticket = store.get_mut(id as usize).and_then(|t| t.take());
        // let mut tk = store.iter().filter_map(|t| {
        //     if t.clone().unwrap().id == id {
        //         return t.clone();
        //     }
        //     None
        // });
        ticket.ok_or(Error::DeleteFailIdNotFound {id})
    }   
}
