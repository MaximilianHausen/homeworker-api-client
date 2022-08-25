use serde::{Deserialize, Serialize};

// Status: Checked but unfinished

#[derive(Serialize, Deserialize, Clone)]
pub struct School {
    pub id: u32,
    pub name: String,
    pub address: Option<String>,
    pub lat: f32,
    //TODO: lat/lon in School "0" default value
    pub lon: f32,
    pub website: Option<String>,
    pub logo: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub director: Option<String>,
    pub region: String,
    pub city: String,
    pub bigbluebutton_participants: Option<String>,
    //TODO: Deserialite bigbluebutton_participants as number
    pub loan: Option<f32>,
    pub verified_by: Option<u32>,
}
