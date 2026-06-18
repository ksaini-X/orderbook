use chrono::format::Pad::Zero;
use rust_decimal::{Decimal, dec};
use shared::{
    engine::order::{Fill, Order},
    error::book::BookError,
};
use std::collections::{BTreeMap, HashMap, VecDeque};
use uuid::Uuid;
pub struct Orderbook {
    pub market: String,
    pub market_id: Uuid,
    pub bids: BTreeMap<Decimal, VecDeque<Order>>,
    pub asks: BTreeMap<Decimal, VecDeque<Order>>,
    pub last_traded_price: Decimal,
}

impl Orderbook {
    pub fn new(market: String) -> Self {
        let market_id = uuid::Uuid::new_v4();
        Self {
            market,
            market_id,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_traded_price: dec!(0),
        }
    }

    pub fn place_order(&mut self, order: Order) -> Result<(Decimal, Vec<Fill>), BookError> {
        let executed_quantity = dec!(0);
        let fills = Vec::<Fill>::new();

        Ok((executed_quantity, fills))
    }
    pub fn match_bids(&mut self, order: Order) -> (Decimal, Vec<Fill>) {
        let executed_quantity = dec!(0);
        let fills = Vec::<Fill>::new();


        (executed_quantity, fills)

        
    }
    pub fn match_asks(&mut self, order: Order) -> (Decimal, Vec<Fill>) {
         let executed_quantity = dec!(0);
        let fills = Vec::<Fill>::new();
        for ask in self.asks.iter_mut(){}

        (executed_quantity, fills)
    }
    }
    pub fn best_ask() -> Option<Decimal> {}
    pub fn best_bid() -> Option<Decimal> {}
    pub fn cancel_order(&mut self, order_id: Uuid) -> Result<Order, BookError> {}
    pub fn get_open_orders_for_user(&mut self, user_id: Uuid) -> Result<Vec<Order>, BookError> {}
    pub fn clean_up() {}
}
