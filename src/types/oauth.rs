use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AccessToken {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String,
    refresh_token: String,
}
