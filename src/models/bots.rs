use chrono;
use serde;
use sqlx;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
// CREATE TABLE bots (
//     id           VARCHAR(36)  PRIMARY KEY,  -- UUID
//     name         VARCHAR(100) NOT NULL,
//     voice_id     VARCHAR(100),              -- ID de voz TTS
//     model_name   VARCHAR(100) NOT NULL,     -- ej: "llama3.2:3b"
//     system_prompt TEXT        NOT NULL,     -- prompt base de personalidad
//     openness     FLOAT        DEFAULT 0.7,
//     sociability  FLOAT        DEFAULT 0.6,
//     retention    FLOAT        DEFAULT 0.8,
//     agreeableness FLOAT       DEFAULT 0.5,
//     volatility   FLOAT        DEFAULT 0.3,
//     loyalty      FLOAT        DEFAULT 0.7,
//     max_ctx_tokens INT        DEFAULT 4096,
//     is_active    BOOLEAN      DEFAULT TRUE,
//     created_at   DATETIME     DEFAULT CURRENT_TIMESTAMP
// );

//I dont know. I thik it would be better to add other files for the models but o dont have any problem with 
pub struct Bot {
    pub id: String,
    pub name: String,
    pub voide_id: String,
    pub model_name: String,
    pub system_prompt: String,
    #[sqlx(json)]
    pub personality_stats: PersonalityStats,
    pub max_ctx_tokens: i32,
    pub is_ative: bool,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct PersonalityStats {
    pub opennes: f32,
    pub sociability: f32,
    pub retention: f32,
    pub agreeableness: f32,
    pub volability: f32,
    pub loyalty: f32,
}
