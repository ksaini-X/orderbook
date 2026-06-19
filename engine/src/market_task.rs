// engine/src/market_task.rs
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::book::book::Orderbook;
use crate::messages::EngineMessage;

pub async fn market_task(
    market_id: Uuid,
    market_name: String,
    mut rx: mpsc::Receiver<EngineMessage>,
) {
    let mut book = Orderbook::new(market_name); // book lives here, nowhere else

    loop {
        match rx.recv().await {
            Some(EngineMessage::PlaceOrder(order)) => {
                match book.place_order(order) {
                    Ok((qty, fills)) => {
                        println!("[{}] filled qty={}, fills={:?}", market_id, qty, fills);
                        // TODO: publish fills to Redis
                        // TODO: publish trade events to Kafka
                    }
                    Err(e) => {
                        println!("[{}] place_order error: {:?}", market_id, e);
                    }
                }
            }

            None => {
                // channel closed — engine dropped the sender
                // this market is being shut down
                println!("[{}] market task shutting down", market_id);
                break;
            }
        }
    }
}
