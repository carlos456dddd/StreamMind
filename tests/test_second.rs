
use StremMind::{models::{self, bot_memories::BotMemories}, pool, queries};
#[tokio::test]

async fn segundo() {

    //Conection
    let conex =  pool::create_pool().await.unwrap();
    
    //For the momento, I only use the fn ->

    let bot_memories = BotMemories{ id: "id_v0".to_string(),bot_id: Some("prueba_01".to_string()), session_id: Some("prueba_01".to_string()), content: "Una_cosa_barbara".to_string(), memory_type: models::bot_memories::MemoryType::Neutral, relevance_score: Some(5.0), memory_source: models::bot_memories::MemorySource::Bot, source_name: Some("Buenos_resultados_v0".to_string()), created_at: Some(chrono::Utc::now().naive_utc())};

    let inser_memory = queries::insert_memory(&conex, &bot_memories).await.unwrap();

    let query_info  = queries::get_stream_info_memories(&conex, &inser_memory).await;
    
    // let primero = !query_info[0];
    assert!(!query_info.is_empty(), "Memories should not be empty");
}

//      let fecha = NaiveDate::from_ymd_opt(2026, 4, 23).unwrap()
//         .and_hms_opt(22, 9, 5).unwrap();
//     assert_eq!(query_info, [models::bot_memories::BotMemories { id: "54bfbb1e-350b-4ab9-a5f2-69fc4a44512c".to_string(), bot_id: Some("2dcaa552-989e-42ec-b90c-4f52c5a122e1".to_string()), session_id: Some("dasd".to_string()), content: "Una_cosa_barbara".to_string(), memory_type: models::bot_memories::MemoryType::Neutral, relevance_score: Some(5.0), memory_source: models::bot_memories::MemorySource::Bot, source_name: Some("Buenos_resultados".to_string()), created_at: Some(fecha) }])





