use uuid::Uuid;

pub enum EngineIncomingEvent {
    CreateMarket {
        client_id: Uuid,
        user_id: Uuid,
        name: String,
    },
    DeleteMarket {
        client_id: Uuid,
        user_id: Uuid,
        market_id: Uuid,
    },
    PauseMarket {
        client_id: Uuid,
        user_id: Uuid,
        market_id: Uuid,
    },
}
pub enum EngineOutGoingEvent {
    MarketCreated {},
}
