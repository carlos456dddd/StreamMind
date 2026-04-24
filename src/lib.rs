

pub mod models;
pub mod pool;
pub mod queries;

// use chrono::{self, Utc};
// use serde;
// use sqlx;

// // CREATE TABLE bots (
// //     id           VARCHAR(36)  PRIMARY KEY,  -- UUID
// //     name         VARCHAR(100) NOT NULL,
// //     voice_id     VARCHAR(100),              -- ID de voz TTS
// //     model_name   VARCHAR(100) NOT NULL,     -- ej: "llama3.2:3b"
// //     system_prompt TEXT        NOT NULL,     -- prompt base de personalidad
// //     openness     FLOAT        DEFAULT 0.7,
// //     sociability  FLOAT        DEFAULT 0.6,
// //     retention    FLOAT        DEFAULT 0.8,
// //     agreeableness FLOAT       DEFAULT 0.5,
// //     volatility   FLOAT        DEFAULT 0.3,
// //     loyalty      FLOAT        DEFAULT 0.7,
// //     max_ctx_tokens INT        DEFAULT 4096,
// //     is_active    BOOLEAN      DEFAULT TRUE,
// //     created_at   DATETIME     DEFAULT CURRENT_TIMESTAMP
// // );

// //I dont know. I thik it would be better to add other files for the models but o dont have any problem with

// #[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
// pub struct Bot {
//     pub id: String,
//     pub name: String,
//     pub voice_id: String,
//     pub model_name: String,
//     pub system_prompt: String,
//     #[sqlx(flatten)]
//     pub personality_stats: PersonalityStats,
//     pub max_ctx_tokens: i32,
//     pub is_active: bool,
//     pub created_at: chrono::NaiveDateTime,
// }

// #[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
// pub struct PersonalityStats {
//     pub openness: f32,
//     pub sociability: f32,
//     pub retention: f32,
//     pub agreeableness: f32,
//     pub volatility: f32,
//     pub loyalty: f32,
// }
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow, PartialEq)]
// pub struct BotMemories {
//     pub id: String,
//     pub bot_id: String,
//     pub session_id: Option<String>,
//     pub content: String,
//     pub memory_type: MemoryType,
//     pub relevance_score: Option<f32>,
//     pub memory_source: MemorySource,
//     pub source_name: Option<String>,
//     pub created_at: Option<chrono::NaiveDateTime>,
// }

// #[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
// #[sqlx(type_name = "memory_type", rename_all = "snake_case")]
// pub enum MemoryType {
//     Neutral,
//     Positive,
//     Negative,
//     StreamInfo,
// }

// #[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
// #[sqlx(type_name = "memory_type", rename_all = "snake_case")]
// pub enum MemorySource {
//     Streamer,
//     Bot,
//     Chat,
//     System,
// }

// pub async fn get_bot_by_id(_pool: &sqlx::MySqlPool, _id: &str) -> Option<Bot> {
//     sqlx::query_as::<_, Bot>("SELECT * FROM bots WHERE id = ?")
//         .bind(_id)
//         .fetch_optional(_pool)
//         .await
//         .ok()
//         .flatten()
// }

// pub async fn get_memories_for_bot(
//     _pool: &sqlx::MySqlPool,
//     _bot_id: &str,
//     _session_id: &str,
//     _limit: i8,
// ) -> Result<Vec<BotMemories>, anyhow::Error> {
//     let result = sqlx::query_as!(
//     BotMemories,
//     r#"SELECT id, bot_id, session_id, memory_type AS "memory_type: MemoryType", content, source AS "memory_source: MemorySource",source_name, relevance_score, created_at
//        FROM bot_memories 
//        WHERE bot_id = ? AND session_id = ? LIMIT ?"#,
//     _bot_id,
//     _session_id,
//     _limit
// )
// .fetch_all(_pool)
// .await?;

//     Ok(result)
// }
// pub async fn insert_memory(
//     pool: &sqlx::MySqlPool,
//     memory: &BotMemories, // Referencia, no ownership
// ) -> Result<String, anyhow::Error> {
//     // Si el ID viene vacío, generamos uno; si no, respetamos el existente

//     let _result = sqlx::query!(
//         "
//         INSERT INTO bot_memories 
//             (id, bot_id, session_id, content, memory_type, 
//              relevance_score, source, source_name, created_at)
//         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
//         ",
//         memory.id,
//         memory.bot_id,
//         memory.session_id,
//         memory.content,
//         memory.memory_type,
//         memory.relevance_score,
//         memory.memory_source,
//         memory.source_name,
//         memory.created_at
//     )
//     .execute(pool)
//     .await
//     .context("Error insertando en la base de datos")?;

//     Ok(memory.id.clone())
// }

// use anyhow::{Context, Ok};
// use dotenvy;
// use sqlx::mysql::MySqlPoolOptions;
// use std::time::Duration;

// pub async fn create_pool() -> Result<sqlx::MySqlPool, anyhow::Error> {
//     let database_url = dotenvy::var("DATABASE_URL")?;
//     let b = database_url.as_str();
//     println!("El valor de de db_es: {}", &database_url);
//     let value = MySqlPoolOptions::new()
//         .max_connections(10)
//         .acquire_timeout(Duration::new(10, 0))
//         .connect(b)
//         .await
//         .context("Error de conexion")?;
//     // pub id: String,
//     // pub name: String,
//     // pub voice_id: String,
//     // pub model_name: String,
//     // pub system_prompt: String,

//     // sqlx::migrate!().run(&value).await?;

//     Ok(value)
// }

// pub async fn bot_sacar() -> Option<Bot> {
//     let a = create_pool().await.unwrap();
//     let resp = get_bot_by_id(&a, "sdasdasdasdasd").await?;

//     Some(resp)
// }

// pub fn results(_val: Option<Bot>) -> Bot {
//     if let Some(l) = _val {
//         return l;
//     } else {
//         return Bot {
//             id: "error".to_string(),
//             name: "error".to_string(),
//             voice_id: "error".to_string(),
//             model_name: "error".to_string(),
//             system_prompt: "error".to_string(),
//             personality_stats: PersonalityStats {
//                 openness: 1.0,
//                 sociability: 1.0,
//                 retention: 1.0,
//                 agreeableness: 1.0,
//                 volatility: 1.0,
//                 loyalty: 1.0,
//             },
//             max_ctx_tokens: 0,
//             is_active: false,
//             created_at: Utc::now().naive_utc(),
//         };
//     }
// }
