use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime};

// Status: Checked but unfinished

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub name_is_editable: bool,
    pub badges: HashMap<String, String>, //TODO: Empty array if no entries?
    pub is_pro: bool,
    pub is_student: bool,
    pub is_teacher: bool,
    pub mail: Option<String>,
    pub mail_verified: bool,
    pub mail_is_editable: bool,
    pub mobile: Option<String>,
    pub mobile_verified: bool,
    pub mobile_is_editable: bool,
    pub birthday: Date,
    pub birthday_is_editable: bool,
    pub registered_at: PrimitiveDateTime,
    pub last_seen_at: PrimitiveDateTime,
    pub avatar_url: url::Url,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessTokenInfo {
    pub token: String,
    pub created_at: PrimitiveDateTime,
    pub expires_at: PrimitiveDateTime,
    pub expires_in: i32,
    #[serde(rename = "type")]
    pub token_type: String, // Enum: Bearer, MAC
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Presence {
    pub is_online: bool,
    pub expires_in: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Navigation {
    pub items: Vec<NavigationEntry>,
    pub cut_after: u16,
    pub branding: NavigationBranding,
    pub me: NavigationUser,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NavigationEntry {
    pub icon: String,
    pub text: String,
    pub link: String,
    pub url_regex: String,
    pub no_request: bool,
    pub badge_identifier: Option<String>, //TODO: Find type of badge_identifier in NavigationEntry (placeholder String)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NavigationBranding {
    pub logo: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NavigationUser {
    pub name: String,
    pub avatar_url: url::Url,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setting {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    pub id: u32,
    pub title: String,
    pub subtitle: Option<String>,
    pub body: String,
    pub document_relation: Option<String>, //TODO: Check type of document_relation in Notification (probably String)
    pub link: url::Url,
    pub tag: Option<()>, //TODO: Find type of tag in Notification
    pub created_at: PrimitiveDateTime,
    pub seen_at: Option<PrimitiveDateTime>,
    pub title_formatted: String,
    pub title_basic: String,
    pub body_formatted: String,
    pub body_basic: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Membership {
    pub course_id: u32,
    pub user_id: u32,
    pub time: i32,
    pub changed: i32, // What is this
    pub is_pending: bool,
    pub is_accepted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StudentInfo {
    pub id: String,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub grade: String,
    pub course: String,
    pub birthday: String,
    pub last_sync: String,
}
