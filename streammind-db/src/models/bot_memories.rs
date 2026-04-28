use chrono;
use serde;
use sqlx;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow, PartialEq)]
pub struct BotMemories {
    pub id: String,
    pub bot_id: Option<String>,
    pub session_id: Option<String>,
    pub content: String,
    pub memory_type: MemoryType,
    pub relevance_score: Option<f32>,
    pub memory_source: MemorySource,
    pub source_name: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,  
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
#[sqlx(type_name = "memory_type", rename_all = "snake_case")]
pub enum MemoryType {
    Neutral,
    Positive,
    Negative,
    StreamInfo,
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
#[sqlx(type_name = "source", rename_all = "snake_case")]
pub enum MemorySource {
    Streamer,
    Bot,
    Chat,
    System,
}
