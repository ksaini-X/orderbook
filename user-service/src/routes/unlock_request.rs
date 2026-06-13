use axum::{Json, extract::State};
use rust_decimal::dec;
use shared::user_service::{error::BalanceError, incoming::UnlockRequest};

use crate::AppState;

pub async fn unlock(
    State(state): State<AppState>,
    Json(payload): Json<UnlockRequest>,
) -> Result<(), BalanceError> {
    if payload.amount <= dec!(0) {
        return Err(BalanceError::InvalidAmount);
    }
    let mut state = state
        .cache
        .write()
        .or_else(|_| Err(BalanceError::StateReadingFailed))?;

    let user = state
        .get_mut(&payload.user_id)
        .ok_or(BalanceError::UserNotFound)?;
    if payload.amount > user.locked_balance {
        return Err(BalanceError::InsufficientFunds);
    }
    user.locked_balance -= payload.amount;
    user.available_balance += payload.amount;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use axum::{Json, extract::State};
    use rust_decimal::dec;
    use shared::user_service::{error::BalanceError, incoming::UnlockRequest};
    use uuid::Uuid;

    use crate::{AppState, routes::unlock_request::unlock, state::user::User};

    fn make_state(user: User) -> AppState {
        let users = Arc::new(RwLock::new(HashMap::new()));
        users.write().unwrap().insert(user.user_id, user);
        AppState { cache: users }
    }
    fn get_user(user_id: Uuid, state: AppState) -> Result<User, BalanceError> {
        let state = state.cache.read().unwrap();
        match state.get(&user_id) {
            None => Err(BalanceError::UserNotFound),
            Some(user) => Ok(user.clone()),
        }
    }

    #[tokio::test]
    async fn test_unlock_moves_to_available_balance() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),

            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let unlock_req = UnlockRequest {
            amount: dec!(500),
            user_id,
        };

        let result = unlock(State(state.clone()), Json(unlock_req)).await;
        assert!(result.is_ok());
        let updated_user = get_user(user_id, state).unwrap();
        assert_eq!(updated_user.locked_balance, dec!(0));
        assert_eq!(updated_user.available_balance, dec!(1500));
    }
    #[tokio::test]
    async fn test_unlock_changes_nothing_for_amount_0() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),

            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let unlock_req: UnlockRequest = UnlockRequest {
            amount: dec!(0),
            user_id,
        };

        let result = unlock(State(state.clone()), Json(unlock_req)).await;
        let updated_user = get_user(user_id, state).unwrap();
        assert_eq!(updated_user.locked_balance, dec!(500));
        assert_eq!(updated_user.available_balance, dec!(1000));
        assert!(matches!(result, Err(BalanceError::InvalidAmount))); // not is_ok
    }
    #[tokio::test]
    async fn test_unlock_fails_for_negative_balance() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),

            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let unlock_req = UnlockRequest {
            amount: dec!(-500),
            user_id,
        };

        let result = unlock(State(state.clone()), Json(unlock_req)).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_unlock_fails_for_amount_greater_than_balance() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),

            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let unlock_req = UnlockRequest {
            amount: dec!(50000),
            user_id,
        };

        let result = unlock(State(state.clone()), Json(unlock_req)).await;
        assert!(result.is_err());
    }
}
