// use anyhow::{Context, Ok};
// use sqlx::mysql::MySqlQueryResult;
// use uuid::Uuid;

// use crate::{models::bots::Bot, pool::create_pool};








fn main()  {

   println!("No se se usa el test");
   //  // let bot = StremMind::bot_sacar().await;
   //  // StremMind::results(bot);
   //     let conex =  pool::create_pool().await.unwrap();

   //  //For the momento, I only use the fn ->

   //  let query_info  = queries::get_stream_info_memories(&conex, "2dcaa552-989e-42ec-b90c-4f52c5a122e1").await;

   //  println!("Tenemos que la información es : {:?}", query_info);

    //Primero insertar info
    //Insertar en mysql
    //Usar la fn para recuperar la información
    //     pub struct Bot {
    //     pub id: String,
    //     pub name: String,
    //     pub voide_id: String,
    //     pub model_name: String,
    //     pub system_prompt: String,
    //     #[sqlx(json)]
    //     pub personality_stats: PersonalityStats,
    //     pub max_ctx_tokens: i32,
    //     pub is_ative: bool,
    //     pub created_at: chrono::NaiveDateTime
    // }

    // #[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
    // pub struct PersonalityStats {
    //     pub opennes: f32,
    //     pub sociability: f32,
    //     pub retention: f32,
    //     pub agreeableness: f32,
    //     pub volability: f32,
    //     pub loyalty: f32,
    // }

    //Insertar primero la parte con el struct
    // -- Add migration script here
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
    // let _id = Uuid::new_v4();
    // let _name = "Intento_v0".to_string();
    // let _voice_id = "id_001".to_string();
    // let _model = "sdasd".to_string();
    // let _model2 = "sdasd".to_string();
    // let pool_new_use_for_the_test = create_pool().await?;

    // let f = sqlx::query!(
    //     "
    
    // INSERT INTO bots(id, name, voice_id, model_name, system_prompt) VALUES (?,?,?,?,?)
    //     ",
    //     _id,
    //     _name,
    //     _voice_id,
    //     _model,
    //     _model2
    // )
    // .execute(&pool_new_use_for_the_test)
    // .await
    // .context("Error en la entrega de datos");

    // println!("Hello, world!");
    // println!(":v");

    // Ok(f)
}

//Test de creat un bot o insertar uno, crear una memoria y luego recuperarlo
