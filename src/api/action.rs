/*use std::error::Error;
use serde_json::{json, Value};
use crate::database::db_connection::{get_user, login};
use crate::models::req_models::{GetUserRequest, LoginRequest, Request};

pub async fn handle_message(text: String) -> Result<Value, Box<dyn Error+ Send + Sync>> {
    let request: Request = serde_json::from_str(&text)?;
    match request.action.as_str() {
        "login" => {
            let login_request: LoginRequest = serde_json::from_str(&request.payload)?;
            login(login_request).await
        }
        "get_user" => {
            let user_id: GetUserRequest = serde_json::from_str(&request.payload)?;
            get_user(user_id).await
        }
        _ => {
            let out_json = json!({"message": "Invalid action".to_string()});
            return Ok(out_json);
        },
    }
}*/

use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use async_trait::async_trait;
use crate::database::db_connection::{get_user, login};
use crate::models::req_models::{GetUserRequest, LoginRequest, Request};
#[async_trait]
trait ActionHandler {
    async fn handle(&self, payload: String) -> Result<Value, Box<dyn Error + Send + Sync>>;
}
struct LoginHandler;
#[async_trait]
impl ActionHandler for LoginHandler {
    async fn handle(&self, payload: String) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let login_request: LoginRequest = serde_json::from_str(&payload)?;
        login(login_request).await
    }
}

struct GetUserHandler;
#[async_trait]
impl ActionHandler for GetUserHandler {
    async fn handle(&self, payload: String) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let get_user_request: GetUserRequest = serde_json::from_str(&payload)?;
        get_user(get_user_request).await
    }
}

pub async fn handle_message(text: String) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let request: Request = serde_json::from_str(&text)?;
    let mut action_map: HashMap<&str, Box<dyn ActionHandler + Send + Sync>> = HashMap::new();
    action_map.insert("login", Box::new(LoginHandler));
    action_map.insert("get_user", Box::new(GetUserHandler));
    if let Some(handler) = action_map.get(request.action.as_str()) {
        handler.handle(request.payload).await
    } else {
        let out_json = json!({"message": "Invalid action".to_string()});
        Ok(out_json)
    }
}
