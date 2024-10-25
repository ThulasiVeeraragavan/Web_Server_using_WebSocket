use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub action: String,
    pub payload: String,
}
#[derive(Deserialize)]
pub struct LoginRequest {
    pub first_name: String,
    pub last_name: String,
}
#[derive(Deserialize)]
pub struct GetUserRequest {
    pub user_id: i32,
}