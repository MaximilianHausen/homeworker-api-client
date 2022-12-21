use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use url::Url;

use super::users::Presence;

//TODO: Redo chats from API responses

/*#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chat {
    pub id: i32,
    pub name: String,
    pub created_at: PrimitiveDateTime,
    pub last_message_at: PrimitiveDateTime,
    pub is_direct: bool,
    pub broadcast_only: bool,
    pub files_allowed: bool,
    pub everybody_can_start_call: bool,
    pub call_open_until: Option<>,
    pub url: Url,
    pub call_is_open: bool,
    pub unread_messages: i32,
    pub is_muted: bool,
    pub is_pinned: bool,
    pub is_admin: bool,
    pub last_message: ChatMessage,
    //pub can_write: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub text: String,
    pub has_files: bool,
    pub has_poll: bool,
    pub meta_type: Option<>,
    pub sent_at: PrimitiveDateTime,
    pub files: Vec<>,
    pub is_own: bool,
    pub can_delete: bool,
    pub reactions: Vec<>,
    pub alphanumeric_id: String,
    pub user: ChatUser,
    pub text_formatted: String,
    pub text_basic: String,
    pub poll: Option<>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatUser {
    pub id: i32,
    pub name: String,
    pub badges: HashMap<String, String>,
    pub presence: UserPresence,
    pub profile_token: String,
    pub avatar_url: Url,
}

/// Untested (likely more values)
pub struct ChatCreateInfo {
    pub name: String,
    pub broadcast_only: bool,
    pub files_allowed: bool,
}

/// Untested
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileAttachment {
    pub id: i32,
    pub hash: String,
    pub sha1_checksum: String,
    pub name: String,
    pub content_type: String,
    pub size: i32,
    pub related_to: String,
    pub uploaded: PrimitiveDateTime,
    pub url: Url,
    pub url_by_hash: Url,
    pub preview_url: Url,
    pub download_url: Url,
    pub can_delete: bool,
    pub human_size: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub alphanumeric_id: String,
}

/// Untested
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PollAttachment {
    pub id: i32,
    pub is_anonymous: bool,
    pub is_multiple_choice: bool,
    pub closes_at: Option<>,
    pub alphanumeric_id: String,
    pub options: Vec<PollAttachmentOption>,
    pub total_votes: i32,
    pub users_voted: Vec<PollAttachmentVote>,
}

/// Untested
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PollAttachmentOption {
    pub id: i32,
    pub text: String,
    pub alphanumeric_id: String,
    pub votes: i32,
    pub is_voted: bool,
}

/// Untested
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PollAttachmentVote {
    pub id: i32,
    pub name: String,
    pub badges: HashMap<String, String>,
}*/
