use chrono;
use serde;
use sqlx;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize,PartialEq, sqlx::FromRow)]
pub struct BotMetrics {
    pub id: u64,
    pub bot_id: String,
    pub session_id: String,
    pub ctx_tokens_used: Option<u32>,
    pub ctx_tokens_max: Option<u32>,
    pub messages_received: u32,
    pub messages_accepted: u32,
    pub messages_rejected: u32,
    pub responses_generated: u32,
    pub recorded_at: chrono::NaiveDateTime,
}
