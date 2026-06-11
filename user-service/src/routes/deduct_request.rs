use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{Json, extract::State};
use rust_decimal::dec;
use shared::user_service::{error::BalanceError, incoming::DeductRequest};
use uuid::Uuid;

use crate::state::user::User;

pub async fn deduct(
    State(state): State<Arc<RwLock<HashMap<Uuid, User>>>>,
    Json(payload): Json<DeductRequest>,
) -> Result<(), BalanceError> {
    if payload.amount <= dec!(0) {
        return Err(BalanceError::InvalidAmount);
    }
    let mut state = state
        .write()
        .or_else(|_| Err(BalanceError::StateReadingFailed))?;

    if !state.contains_key(&payload.from) || !state.contains_key(&payload.to) {
        return Err(BalanceError::UserNotFound);
    }

    if state.get(&payload.from).unwrap().available_balance > payload.amount {
        return Err(BalanceError::InsufficientFunds);
    }
    state.get_mut(&payload.from).unwrap().available_balance -= payload.amount;
    state.get_mut(&payload.to).unwrap().available_balance += payload.amount;

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
    use shared::user_service::{error::BalanceError, incoming::LockRequest};
    use uuid::Uuid;

    use crate::{routes::lock_request::lock, state::user::User};

    fn make_state(user: User) -> Arc<RwLock<HashMap<Uuid, User>>> {
        let users = Arc::new(RwLock::new(HashMap::new()));
        users.write().unwrap().insert(user.user_id, user);
        users
    }
    fn get_user(
        user_id: Uuid,
        state: Arc<RwLock<HashMap<Uuid, User>>>,
    ) -> Result<User, BalanceError> {
        let state = state.read().unwrap();
        match state.get(&user_id) {
            None => Err(BalanceError::UserNotFound),
            Some(user) => Ok(user.clone()),
        }
    }

    #[tokio::test]
    async fn test_lock_moves_to_locked_balance() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),
            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let lock_req = LockRequest {
            amount: dec!(500),
            user_id,
        };

        let result = lock(State(state.clone()), Json(lock_req)).await;
        assert!(result.is_ok());
        let updated_user = get_user(user_id, state).unwrap();
        assert_eq!(updated_user.locked_balance, dec!(1000));
        assert_eq!(updated_user.available_balance, dec!(500));
    }
    #[tokio::test]
    async fn test_lock_changes_nothing_for_amount_0() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),
            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let lock_req: LockRequest = LockRequest {
            amount: dec!(0),
            user_id,
        };

        let result = lock(State(state.clone()), Json(lock_req)).await;
        let updated_user = get_user(user_id, state).unwrap();
        assert_eq!(updated_user.locked_balance, dec!(500));
        assert_eq!(updated_user.available_balance, dec!(1000));
        assert!(matches!(result, Err(BalanceError::InvalidAmount))); // not is_ok
    }
    #[tokio::test]
    async fn test_lock_fails_for_negative_balance() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),
            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let lock_req = LockRequest {
            amount: dec!(-500),
            user_id,
        };

        let result = lock(State(state.clone()), Json(lock_req)).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_lock_fails_for_amount_greater_than_balance() {
        let user_id = Uuid::new_v4();
        let user = User {
            available_balance: dec!(1000),
            locked_balance: dec!(500),
            user_id,
        };
        let state = make_state(user);
        let lock_req = LockRequest {
            amount: dec!(50000),
            user_id,
        };

        let result = lock(State(state.clone()), Json(lock_req)).await;
        assert!(result.is_err());
    }
}
