use std::net::IpAddr;

use serde::{Deserialize, Serialize};

pub mod chats;
pub mod lessons;
pub mod schools;
pub mod timetables;
pub mod todos;
pub mod users;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    pub error: Error,
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        Self { error: value }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub name: String,
    pub message: String,
    pub code: u16,
}

impl From<ErrorResponse> for Error {
    fn from(value: ErrorResponse) -> Self {
        value.error
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ips {
    pub dns: Vec<IpAddr>,
    pub webhooks: Vec<IpAddr>,
}
