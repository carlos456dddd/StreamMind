use StremMind::{pool,queries,models};
use chrono::Timelike;
use uuid::Uuid;

#[tokio::test]
async fn test_insert_create_recovery_first() {
    //Insertar un bot
    let conex = pool::create_pool().await.unwrap();

    //Prueba de INSERTAR DATOS
    let _id = Uuid::new_v4().to_string();
    let _name = "Intento_v1_test".to_string();
    let _voice_id = "id_002_TEST".to_string();
    let _model = "sdasd_test".to_string();
    let _model2 = "sdasd_test".to_string();

    let _f = sqlx::query!(
        "
    
    INSERT INTO bots(id, name, voice_id, model_name, system_prompt) VALUES (?,?,?,?,?)
        ",
        &_id,
        &_name,
        &_voice_id,
        &_model,
        &_model2
    )
    .execute(&conex)
    .await
    .unwrap();

    let bot_usado = queries::get_bot_by_id(&conex, _id.as_str())
        .await
        .unwrap();

    // #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
    // pub struct BotMemories {
    //     pub id: String,
    //     pub bot_id: String,
    //     pub session_id: Option<String>,
    //     pub content: String,
    //     #[sqlx(flatten)]
    //     pub memory_type: MemoryType,
    //     pub relevance_score: f32,
    //     #[sqlx(flatten)]
    //     pub memory_source: MemorySource,
    //     pub source_name: Option<String>,
    //     pub created_at: chrono::NaiveDateTime,
    // }

    // #[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
    // #[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
    // pub enum MemoryType {
    //     Neutral,
    //     Positive,
    //     Negative,
    //     StreamInfo,
    // }

    // #[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize, PartialEq)]
    // #[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
    // pub enum MemorySource {
    //     Streamer,
    //     Bot,
    //     Chat,
    //     System,
    // }

    //Crear una memoria
    //Datos de Uuid
    let data = chrono::Utc::now().naive_utc().with_nanosecond(0).unwrap();

    let struct_memory_bot = models::bot_memories::BotMemories {
        id: Uuid::new_v4().to_string(),
        bot_id: Some(bot_usado.id),
        session_id: Some("dasd".to_string()),
        content: "Una_cosa_barbara".to_string(),
        memory_type: models::bot_memories::MemoryType::Neutral,
        relevance_score: Some(5.0),
        memory_source: models::bot_memories::MemorySource::Bot,
        source_name: Some("Buenos_resultados".to_string()),
        created_at: Some(data),
    };

    let _id_memory_bot = queries::insert_memory(&conex, &struct_memory_bot)
        .await
        .unwrap();

    //Recuperar datos de la memoria
    let dato_id = struct_memory_bot.bot_id.clone();
    
    let _datos_memories_bot =
        queries::get_memories_for_bot(&conex,dato_id.unwrap().as_str(),"dasd", 1)
            .await
            .unwrap();

    assert_eq!(_datos_memories_bot[0], struct_memory_bot);

    // let mut verdad = true;
    // if struct_memory_bot == _datos_memories_bot[0] {

    //     verdad = true;
    // }    else {
    //     verdad = false;
    // }

    // assert!(verdad);
    // let _datos: StremMind::BotMemories = sqlx::query_as("SELECT * FROM bot_memories WHERE id = ?")
    //     .bind(&_id_memory_bot)
    //     .fetch_one(&conex)
    //     .await
    //     .unwrap();

    // let f = StremMind::bot_sacar().await; //Me esta dando algo, cosa sería jaaa
    // assert!(f.is_some_and(|x| x.id == "sdasdasdasdasd".to_string()));
}
