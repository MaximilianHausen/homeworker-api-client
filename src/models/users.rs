use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub mail: String,
    pub mail_verified: bool,
    pub mobile: String,
    pub mobile_verified: bool,
    pub registered_at: PrimitiveDateTime,
    pub last_seen_at: PrimitiveDateTime,
    pub is_pro: bool,
    pub is_student: bool,
    pub is_teacher: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccessToken {
    pub token: String,
    pub created_at: PrimitiveDateTime,
    pub expires_at: PrimitiveDateTime,
    pub expires_in: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Presence {
    pub is_online: bool,
    pub expires_in: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Membership {
    pub course_id: i32,
    pub user_id: i32,
    pub time: i32,
    pub changed: i32,
    pub is_pending: bool,
    pub is_accepted: bool,
}
