use chrono::Utc;
use rust_decimal::{Decimal, dec};
use shared::{
    engine::order::{Fill, Order},
    error::book::BookError,
};
use std::{
    cmp::min,
    collections::{BTreeMap, VecDeque},
};
use uuid::Uuid;
pub struct Orderbook {
    pub market: String,
    pub market_id: Uuid,
    pub bids: BTreeMap<Decimal, VecDeque<Order>>,
    pub asks: BTreeMap<Decimal, VecDeque<Order>>,
    pub last_traded_price: Decimal,
}
use shared::engine::order::Side;

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

    pub fn place_order(&mut self, mut order: Order) -> Result<(Decimal, Vec<Fill>), BookError> {
        let (executed_quantity, fills) = match order.side {
            Side::Ask => self.match_bids(&mut order),
            Side::Bid => self.match_asks(&mut order),
        };

        if order.filled_quantity < order.quantity {
            match order.side {
                Side::Ask => self
                    .asks
                    .entry(order.price)
                    .or_insert_with(VecDeque::new)
                    .push_back(order),
                Side::Bid => self
                    .bids
                    .entry(order.price)
                    .or_insert_with(VecDeque::new)
                    .push_back(order),
            }
        }

        if executed_quantity > dec!(0) {
            self.last_traded_price = fills.last().unwrap().price;
        }

        Ok((executed_quantity, fills))
    }
    pub fn match_bids(&mut self, order: &mut Order) -> (Decimal, Vec<Fill>) {
        let mut executed_quantity = dec!(0);
        let mut fills = Vec::<Fill>::new();
        //Order {p:100, q:10, s: Ask} Order wants to sell q:10 at p:100 each

        /*
        bids [
            {p:99, q:10}, -> want to buy q:10 for q:99b
            {p:98, q:11}
        ]
         */
        for (price, bids) in (self.bids.iter_mut()).rev() {
            if order.price > *price {
                break;
            }
            while !bids.is_empty() && order.filled_quantity < order.quantity {
                let bid = bids.front_mut().unwrap();
                let filled_quantity = min(
                    bid.quantity - bid.filled_quantity,
                    order.quantity - order.filled_quantity,
                );
                executed_quantity += filled_quantity;
                order.filled_quantity += filled_quantity;
                bid.filled_quantity += filled_quantity;

                let fill = Fill {
                    maker_order_id: bid.order_id,
                    taker_order_id: order.order_id,
                    maker_user_id: bid.user_id,
                    taker_user_id: order.user_id,
                    market_id: order.market_id,
                    price: bid.price,
                    quantity: filled_quantity,
                    timestamp: Utc::now(),
                };
                fills.push(fill);

                if bid.filled_quantity == bid.quantity {
                    bids.pop_front();
                }
            }
        }

        self.bids.retain(|_p, bids| !bids.is_empty());

        (executed_quantity, fills)
    }
    pub fn match_asks(&mut self, order: &mut Order) -> (Decimal, Vec<Fill>) {
        let mut executed_quantity = dec!(0);
        let mut fills = Vec::<Fill>::new();
        /*
        Order { p:100, q:10, s:Bid} - wants to buy q:10 at p:100

        Asks[
            {q:10, p:99}
            {q:10, p:100}
            {q:10, p:101}
        ]
         */
        for (price, asks) in self.asks.iter_mut() {
            if *price > order.price {
                break;
            }
            while !asks.is_empty() && order.filled_quantity < order.quantity {
                let ask = asks.front_mut().unwrap();

                let filled_quantity = min(
                    ask.quantity - ask.filled_quantity,
                    order.quantity - order.filled_quantity,
                );
                executed_quantity += filled_quantity;
                order.filled_quantity += filled_quantity;
                ask.filled_quantity += filled_quantity;

                let fill = Fill {
                    maker_order_id: ask.order_id,
                    taker_order_id: order.order_id,
                    maker_user_id: ask.user_id,
                    taker_user_id: order.user_id,
                    market_id: order.market_id,
                    price: ask.price,
                    quantity: filled_quantity,
                    timestamp: Utc::now(),
                };
                fills.push(fill);

                if ask.filled_quantity == ask.quantity {
                    asks.pop_front();
                }
            }
        }
        self.asks.retain(|_p, asks| !asks.is_empty());
        (executed_quantity, fills)
    }

    pub fn best_ask(&self) -> Option<Decimal> {
        self.asks.first_key_value().map(|(price, _)| *price)
    }

    pub fn best_bid(&self) -> Option<Decimal> {
        self.bids.last_key_value().map(|(price, _)| *price)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use shared::engine::order::{Order, Side, Status};
    use uuid::Uuid;

    fn create_order(side: Side, price: Decimal, quantity: Decimal) -> Order {
        Order {
            order_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            market_id: Uuid::new_v4(),
            side,
            price,
            quantity,
            filled_quantity: dec!(0),
            status: Status::Pending,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn bid_order_added_when_no_asks() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let bid = create_order(Side::Bid, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(bid).unwrap();

        assert_eq!(executed, dec!(0));
        assert!(fills.is_empty());

        assert_eq!(book.best_bid(), Some(dec!(100)));
        assert_eq!(book.best_ask(), None);

        assert_eq!(book.bids[&dec!(100)].len(), 1);
    }

    #[test]
    fn ask_order_added_when_no_bids() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let ask = create_order(Side::Ask, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(ask).unwrap();

        assert_eq!(executed, dec!(0));
        assert!(fills.is_empty());

        assert_eq!(book.best_bid(), None);
        assert_eq!(book.best_ask(), Some(dec!(100)));

        assert_eq!(book.asks[&dec!(100)].len(), 1);
    }

    #[test]
    fn bid_matches_single_ask_exact_quantity() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let ask = create_order(Side::Ask, dec!(100), dec!(10));
        book.place_order(ask).unwrap();

        let bid = create_order(Side::Bid, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(bid).unwrap();

        assert_eq!(executed, dec!(10));
        assert_eq!(fills.len(), 1);

        assert_eq!(fills[0].price, dec!(100));
        assert_eq!(fills[0].quantity, dec!(10));

        assert!(book.asks.is_empty());
        assert_eq!(book.last_traded_price, dec!(100));
    }

    #[test]
    fn ask_matches_single_bid_exact_quantity() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let bid = create_order(Side::Bid, dec!(100), dec!(10));
        book.place_order(bid).unwrap();

        let ask = create_order(Side::Ask, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(ask).unwrap();

        assert_eq!(executed, dec!(10));
        assert_eq!(fills.len(), 1);

        assert_eq!(fills[0].price, dec!(100));
        assert_eq!(fills[0].quantity, dec!(10));

        assert!(book.bids.is_empty());
        assert_eq!(book.last_traded_price, dec!(100));
    }

    #[test]
    fn bid_partially_fills_larger_ask() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let ask = create_order(Side::Ask, dec!(100), dec!(20));
        book.place_order(ask).unwrap();

        let bid = create_order(Side::Bid, dec!(100), dec!(5));

        let (executed, fills) = book.place_order(bid).unwrap();

        assert_eq!(executed, dec!(5));
        assert_eq!(fills.len(), 1);

        let remaining = &book.asks[&dec!(100)][0];

        assert_eq!(remaining.quantity, dec!(20));
        assert_eq!(remaining.filled_quantity, dec!(5));
    }

    #[test]
    fn ask_partially_fills_larger_bid() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let bid = create_order(Side::Bid, dec!(100), dec!(20));
        book.place_order(bid).unwrap();

        let ask = create_order(Side::Ask, dec!(100), dec!(5));

        let (executed, fills) = book.place_order(ask).unwrap();

        assert_eq!(executed, dec!(5));
        assert_eq!(fills.len(), 1);

        let remaining = &book.bids[&dec!(100)][0];

        assert_eq!(remaining.quantity, dec!(20));
        assert_eq!(remaining.filled_quantity, dec!(5));
    }

    #[test]
    fn bid_consumes_multiple_asks_same_price() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        book.place_order(create_order(Side::Ask, dec!(100), dec!(5)))
            .unwrap();
        book.place_order(create_order(Side::Ask, dec!(100), dec!(5)))
            .unwrap();

        let bid = create_order(Side::Bid, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(bid).unwrap();

        assert_eq!(executed, dec!(10));
        assert_eq!(fills.len(), 2);

        assert!(book.asks.is_empty());
    }

    #[test]
    fn ask_consumes_multiple_bids_same_price() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        book.place_order(create_order(Side::Bid, dec!(100), dec!(5)))
            .unwrap();
        book.place_order(create_order(Side::Bid, dec!(100), dec!(5)))
            .unwrap();

        let ask = create_order(Side::Ask, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(ask).unwrap();

        assert_eq!(executed, dec!(10));
        assert_eq!(fills.len(), 2);

        assert!(book.bids.is_empty());
    }

    #[test]
    fn bid_consumes_multiple_price_levels() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        book.place_order(create_order(Side::Ask, dec!(99), dec!(5)))
            .unwrap();
        book.place_order(create_order(Side::Ask, dec!(100), dec!(5)))
            .unwrap();
        book.place_order(create_order(Side::Ask, dec!(101), dec!(5)))
            .unwrap();

        let bid = create_order(Side::Bid, dec!(101), dec!(12));

        let (executed, fills) = book.place_order(bid).unwrap();

        assert_eq!(executed, dec!(12));
        assert_eq!(fills.len(), 3);

        let remaining = &book.asks[&dec!(101)][0];

        assert_eq!(remaining.filled_quantity, dec!(2));
    }

    #[test]
    fn bid_does_not_match_higher_ask_price() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        book.place_order(create_order(Side::Ask, dec!(101), dec!(10)))
            .unwrap();

        let bid = create_order(Side::Bid, dec!(100), dec!(10));

        let (executed, fills) = book.place_order(bid).unwrap();

        assert_eq!(executed, dec!(0));
        assert!(fills.is_empty());

        assert_eq!(book.best_bid(), Some(dec!(100)));
        assert_eq!(book.best_ask(), Some(dec!(101)));
    }

    #[test]
    fn fifo_priority_respected_for_same_price_asks() {
        let mut book = Orderbook::new("BTCUSDT".to_string());

        let ask1 = create_order(Side::Ask, dec!(100), dec!(5));
        let ask1_id = ask1.order_id;

        let ask2 = create_order(Side::Ask, dec!(100), dec!(5));
        let ask2_id = ask2.order_id;

        book.place_order(ask1).unwrap();
        book.place_order(ask2).unwrap();

        let bid = create_order(Side::Bid, dec!(100), dec!(10));

        let (_, fills) = book.place_order(bid).unwrap();

        assert_eq!(fills.len(), 2);

        assert_eq!(fills[0].maker_order_id, ask1_id);
        assert_eq!(fills[1].maker_order_id, ask2_id);
    }

    #[test]
    fn best_bid_returns_none_for_empty_book() {
        let book = Orderbook::new("BTCUSDT".to_string());

        assert_eq!(book.best_bid(), None);
    }

    #[test]
    fn best_ask_returns_none_for_empty_book() {
        let book = Orderbook::new("BTCUSDT".to_string());

        assert_eq!(book.best_ask(), None);
    }
}
