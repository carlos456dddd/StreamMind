use anyhow::{Context, Ok};
use sqlx::mysql::MySqlQueryResult;

use crate::models::bot_memories::{MemorySource, MemoryType};
use crate::models::bots::PersonalityStats;
use crate::models::{
    bot_memories::BotMemories, bot_metrics::BotMetrics, bots::Bot, interaction::Interaction,
};

pub async fn get_active_bots(_pool: &sqlx::MySqlPool) -> Result<Vec<Bot>, anyhow::Error> {
    // let pool = sqlx::mysql::MySqlPoolOptions::new()
    // .max_connections(5).connect("postgres://postgres:password@localhost/test").await?;
    //Lo sospechaba pero, el tipo que esta dando es justamente un type integrado que reconoce al &sqlx::MySqlPool
    //Simplemente estamos dando conexión y sacando el valor que necesitamos

    let result = sqlx::query!("SELECT * FROM bots WHERE is_active = ?", true)
        .fetch_all(_pool)
        .await?;

    let f = result
        .into_iter()
        .map(|bot| Bot {
            id: bot.id,
            name: bot.name,
            voice_id: bot.voice_id,
            model_name: bot.model_name,
            system_prompt: bot.system_prompt,
            personality_stats: PersonalityStats {
                openness: bot.openness,
                sociability: bot.sociability,
                retention: bot.retention,
                agreeableness: bot.agreeableness,
                volatility: bot.volatility,
                loyalty: bot.loyalty,
            },
            max_ctx_tokens: bot.max_ctx_tokens,
            is_active: Some(bot.is_active != Some(0)),
            created_at: bot.created_at,
        })
        .collect();

    Ok(f)
    //Supondría que primero estamos obteniendo la conexión y luego se relaiza la llamada con la query correspondiente
    // let _bots=_pool.fetch_all("SELECT * FROM bots WHERE is_active = true ");
}

pub async fn get_bot_by_id(_pool: &sqlx::MySqlPool, _id: &str) -> Bot {
    let bot = sqlx::query!(r#"SELECT * FROM bots WHERE id = ? "#, _id)
        .fetch_one(_pool)
        .await
        .unwrap();
    let bools = bot.is_active != Some(0);
    Bot {
        id: bot.id,
        name: bot.name,
        voice_id: bot.voice_id,
        model_name: bot.model_name,
        system_prompt: bot.system_prompt,
        personality_stats: PersonalityStats {
            openness: bot.openness,
            sociability: bot.sociability,
            retention: bot.retention,
            agreeableness: bot.agreeableness,
            volatility: bot.volatility,
            loyalty: bot.loyalty,
        },
        max_ctx_tokens: bot.max_ctx_tokens,
        is_active: Some(bools),
        created_at: bot.created_at,
    }
}

pub async fn insert_memory(
    pool: &sqlx::MySqlPool,
    memory: &BotMemories, // Referencia, no ownership
) -> Result<String, anyhow::Error> {
    // Si el ID viene vacío, generamos uno; si no, respetamos el existente

    let _result = sqlx::query!(
        "
        INSERT INTO bot_memories 
            (id, bot_id, session_id, content, memory_type, 
             relevance_score, source, source_name, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ",
        memory.id,
        memory.bot_id,
        memory.session_id,
        memory.content,
        memory.memory_type,
        memory.relevance_score,
        memory.memory_source,
        memory.source_name,
        memory.created_at
    )
    .execute(pool)
    .await
    .context("Error insertando en la base de datos")?;

    Ok(memory.id.clone())
}

pub async fn get_memories_for_bot(
    _pool: &sqlx::MySqlPool,
    _bot_id: &str,
    _session_id: &str,
    _limit: i8,
) -> Result<Vec<BotMemories>, anyhow::Error> {
    let result = sqlx::query_as!(
    BotMemories,
    r#"SELECT id, bot_id, session_id, memory_type AS "memory_type: MemoryType", content, source AS "memory_source: MemorySource",source_name, relevance_score, created_at
       FROM bot_memories 
       WHERE bot_id = ? AND session_id = ? LIMIT ?"#,
    _bot_id,
    _session_id,
    _limit
)
.fetch_all(_pool)
.await?;

    Ok(result)
}

pub async fn get_stream_info_memories(_pool: &sqlx::MySqlPool, _bot_id: &str) -> Vec<BotMemories> {
    //Lo que se quiere es tener los que tengan el stream info del bot especifico

    let _result: Vec<BotMemories> = sqlx::query_as!(
        BotMemories,
        r#"SELECT id, bot_id, session_id, memory_type AS "memory_type: MemoryType", content, source AS "memory_source: MemorySource",source_name, relevance_score, created_at
       FROM bot_memories 
     WHERE bot_id = ? AND memory_type = 'neutral'"#,
        _bot_id
    )
    .fetch_all(_pool)
    .await
    .unwrap();

    //No sabía que resitrabael compilador solo la parte de el vector parece que solo pide eso para pasar desapercibido
    _result
}

pub async fn upsert_bot_metrics(
    _pool: &sqlx::MySqlPool,
    _metrics: BotMetrics,
) -> Result<MySqlQueryResult, anyhow::Error> {
    //Se hace una insersión de datos
    //En la base de datos se realiza ya la parte de id
    //  ctx_tokens_used INT,
    // ctx_tokens_max  INT,
    // messages_received INT     DEFAULT 0,
    // messages_accepted INT     DEFAULT 0,
    // messages_rejected INT     DEFAULT 0,
    // responses_generated INT   DEFAULT 0,

    // pub id: u128,
    // pub bot_id: String,
    // pub session_id: String,
    // pub ctx_tokens_used: Option<u32>,
    // pub ctx_tokens_max: Option<u32>,
    // pub messages_received: u32,
    // pub messages_accepted: u32,
    // pub messages_rejected: u32,
    // pub responses_generated: u32,
    // pub recorded_at: chrono::NaiveDateTime,

    let _result = sqlx::query!(
        "
        UPDATE 
        bot_metrics 
        SET     ctx_tokens_used = ?,
                ctx_tokens_max  = ?,
                messages_received = ?,
                messages_accepted = ?,
                messages_rejected = ?,
                responses_generated = ?,
                recorded_at = ?
        WHERE 
            id = ?",
        _metrics.ctx_tokens_used,
        _metrics.ctx_tokens_max,
        _metrics.messages_received,
        _metrics.messages_accepted,
        _metrics.messages_rejected,
        _metrics.responses_generated,
        _metrics.recorded_at, //Sería mejor gestionar en otro lado, pero bueno
        _metrics.id
    )
    .execute(_pool)
    .await
    .context("No se completo la actualización de los datos")?;

    Ok(_result)
}

pub async fn insert_interaction(
    _pool: &sqlx::MySqlPool,
    _interaction: &Interaction,
) -> Result<String, anyhow::Error> {
    let _result = sqlx::query!(
        "
        INSERT INTO interactions
        (id, 
        session_id,
        sender_type, 
        sender_id, 
        sender_name, 
        content, 
        response_bot_id, 
        response_content, 
        filter_decision, 
        filter_reason)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        _interaction.id,
        _interaction.session_id,
        _interaction.sender_type,
        _interaction.sender_id,
        _interaction.sender_name,
        _interaction.content,
        _interaction.response_bot_id,
        _interaction.response_content,
        _interaction.filter_desition,
        _interaction.filter_reason
    )
    .execute(_pool)
    .await
    .context("Error en la inserción de datos");

    Ok(_interaction.id.clone())

    //El caso en que sea nulo uno de los enums ya tiene información por defecto
}

//         id           VARCHAR(36)  PRIMARY KEY,
// session_id   VARCHAR(36)  REFERENCES stream_sessions(id),
// sender_type  ENUM('streamer', 'chat_user', 'bot') NOT NULL,
// sender_id    VARCHAR(36),
// sender_name  VARCHAR(100),
// content      TEXT         NOT NULL,
// response_bot_id VARCHAR(36) REFERENCES bots(id),
// response_content TEXT,
// filter_decision ENUM('accepted', 'rejected', 'neutral') DEFAULT 'accepted',
// filter_reason VARCHAR(255),
// created_at   DATETIME     DEFAULT CURRENT_TIMESTAMP

////Información  del struct
//     pub struct Interaction {
//     pub id: String,
//     pub session_id: String,
//     pub sender_type: SenderType,
//     pub sender_id: Option<String>,
//     pub sender_name: Option<String>,
//     pub content: Option<String>,
//     pub response_bot_id: Option<String>,
//     pub response_content: Option<String>,
//     pub filter_reason: Option<String>,
// }
