use uuid::Uuid;

pub struct RegisterRequestData {
    email: String,
    password: String,
    name: String,
}
pub struct RegisterResponseData {
    user_id: Uuid,
}
pub async fn register() {}
