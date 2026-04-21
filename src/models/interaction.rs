use serde;
use sqlx;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Interaction {
    pub id: String,
    pub session_id: String,
    pub sender_type: SenderType,
    pub sender_id: Option<String>,
    pub sender_name: Option<String>,
    pub content: Option<String>,
    pub response_bot_id: Option<String>,
    pub filter_desition: String,
    pub response_content: Option<String>,
    pub filter_reason: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum SenderType {
    Streamer,
    ChatUser,
    Bot,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum FilterDestion {
    Accepted,
    Rejected,
    Neutral,
}
