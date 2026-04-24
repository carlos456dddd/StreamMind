use chrono;
use serde;
use sqlx;


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

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Bot {
    pub id: String,
    pub name: String,
    pub voice_id: Option<String>,
    pub model_name: String,
    pub system_prompt: String,
    #[sqlx(flatten)]
    pub personality_stats: PersonalityStats,
    pub max_ctx_tokens: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PersonalityStats {
    pub openness: Option<f32>,
    pub sociability: Option<f32>,
    pub retention: Option<f32>,
    pub agreeableness: Option<f32>,
    pub volatility: Option<f32>,
    pub loyalty: Option<f32>,
}

