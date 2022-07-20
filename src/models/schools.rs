use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct School {
    id: i32,
    name: String,
    address: String,
    lat: f32,
    lon: f32,
    website: String,
    logo: String,
    phone: String,
    email: String,
    director: String,
    region: String,
    city: String,
    //loan: null,
    //verified_by: null
}
