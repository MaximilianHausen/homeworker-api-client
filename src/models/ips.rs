use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Ips {
    pub dns: Vec<IpAddr>,
    pub webhooks: Vec<IpAddr>,
}
