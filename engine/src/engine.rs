use crate::{error::EngineError, market_task::market_task, messages::EngineMessage};
use chrono::{DateTime, Utc};
use shared::engine::order::Order;
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Market {
    pub market_id: Uuid,
    pub market_name: String,
    pub sender: mpsc::Sender<EngineMessage>,
    pub created_at: DateTime<Utc>,
    pub creator: Uuid,
    pub active: bool,
}

pub struct Engine {
    pub markets: HashMap<Uuid, Market>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            markets: HashMap::new(),
        }
    }

    pub fn create_market(
        &mut self,
        market_name: String,
        creator_id: Uuid,
    ) -> (Uuid, DateTime<Utc>) {
        let (tx, rx) = mpsc::channel::<EngineMessage>(1024);
        let market_id = Uuid::new_v4();
        let timestamp = Utc::now();

        self.markets.insert(
            market_id,
            Market {
                market_name: market_name.clone(),
                market_id,
                sender: tx,
                created_at: timestamp,
                creator: creator_id,
                active: true,
            },
        );

        tokio::spawn(market_task(market_id, market_name, rx));
        (market_id, timestamp)
    }

    pub fn delete_market(&mut self, market_id: Uuid) -> Result<DateTime<Utc>, EngineError> {
        self.markets
            .remove(&market_id)
            .ok_or(EngineError::MarketNotFound)?;
        Ok(Utc::now())
    }

    pub fn pause_market(&mut self, market_id: Uuid) -> Result<DateTime<Utc>, EngineError> {
        let (_, market) = self
            .markets
            .iter_mut()
            .find(|(&m_id, _)| m_id == market_id)
            .ok_or(EngineError::MarketNotFound)?;

        market.active = false;
        Ok(Utc::now())
    }
}
