use std::net::IpAddr;

use serde::{Deserialize, Serialize};

pub mod chats;
pub mod lessons;
pub mod oauth;
pub mod schools;
pub mod todos;
pub mod users;

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Error {
    pub name: String,
    pub message: String,
    pub code: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ips {
    pub dns: Vec<IpAddr>,
    pub webhooks: Vec<IpAddr>,
}
