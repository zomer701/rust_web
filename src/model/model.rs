use crate::{ctx::{Ctx}, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    ticker_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticker_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_cf: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticker_store.lock().unwrap();    
        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_cf.title,
        };

        store.push(Some(ticket.clone()));

       Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticker_store.lock().unwrap();

        let tickets = store
        .iter()
        .filter_map(|t| t.clone())
        .collect();
        
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {

        let mut store = self.ticker_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
