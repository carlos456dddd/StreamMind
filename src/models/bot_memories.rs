use chrono;
use serde;
use sqlx;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct BotMemories {
    pub id: String,
    pub bot_id: String,
    pub session_id: Option<String>,
    pub content: String,
    pub memory_type: MemoryType,
    pub relevance_score: f32,
    pub memory_source: MemorySource,
    pub source_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum MemoryType {
    Neutral,
    Positive,
    Negative,
    StreamInfo,
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum MemorySource {
    Neutral,
    Positive,
    Negative,
    StreamInfo,
}
