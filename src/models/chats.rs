use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: i32,
    pub name: String,
    pub created_at: PrimitiveDateTime,
    pub last_message_at: PrimitiveDateTime,
    pub broadcast_only: bool,
    pub files_allowed: bool,
    pub url: String,
    pub is_admin: bool,
    pub can_write: bool,
    pub unread_messages: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub user: MessageAuthor,
    pub text: String,
    pub has_files: bool,
    //pub meta_type: null,
    pub sent_at: PrimitiveDateTime,
    pub is_own: bool,
    //pub reactions: []
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageAuthor {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileAttachment {
    pub id: i32,
    pub hash: String,
    pub sha1_checksum: String,
    pub name: String,
    pub content_type: String,
    pub size: i32,
    pub related_to: String,
    pub uploaded: PrimitiveDateTime,
    pub url: String,
    pub url_by_hash: String,
    pub preview_url: String,
    pub download_url: String,
    pub can_delete: bool,
    pub human_size: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub alphanumeric_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PollAttachment {
    pub id: i32,
    pub is_anonymous: bool,
    pub is_multiple_choice: bool,
    //pub closes_at: null,
    pub alphanumeric_id: String,
    pub options: Vec<PollAttachmentOption>,
    pub total_votes: i32,
    pub users_voted: Vec<PollAttachmentVote>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PollAttachmentOption {
    pub id: i32,
    pub text: String,
    pub alphanumeric_id: String,
    pub votes: i32,
    pub is_voted: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PollAttachmentVote {
    pub id: i32,
    pub name: String,
    pub badges: HashMap<String, String>,
}
