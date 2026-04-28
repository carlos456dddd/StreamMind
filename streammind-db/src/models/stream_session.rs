use chrono;
use serde;
use sqlx;


#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct StreamSession {
    pub id: String,
    pub stream_tittle: String,
    pub youtube_chat_id: String,
    pub started_at: chrono::NaiveDateTime,
    pub ended_at: chrono::NaiveDateTime
    
}