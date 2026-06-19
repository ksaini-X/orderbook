use std::collections::HashMap;

use shared::engine::order::Order;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{market_task::market_task, messages::EngineMessage};

#[derive(Debug, Clone)]
pub struct Market {
    pub market_id: Uuid,
    pub market_name: String,
    pub sender: mpsc::Sender<EngineMessage>,
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
    pub fn create_market(&mut self, market_name: String) -> Uuid {
        let (tx, rx) = mpsc::channel::<EngineMessage>(1024);
        let market_id = Uuid::new_v4();
        let market = Market {
            market_name: market_name.clone(),
            market_id,
            sender: tx,
        };
        self.markets.insert(market_id, market);

        tokio::spawn(market_task(market_id, market_name, rx));

        market_id
    }

    pub async fn place_order(&self, market_id: Uuid, order: Order) {
        if let Some(market) = self.markets.get(&market_id) {
            market
                .sender
                .send(EngineMessage::PlaceOrder(order))
                .await
                .ok();
        }
    }
}
