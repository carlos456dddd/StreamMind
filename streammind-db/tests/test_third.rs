use streammind_db::pool::create_pool;
use std::time::Duration;


//En este caso lo hice con chatgpt, pero la cosa en este caso es entender exactamente como esta funcionando, hubiera preferido romper todo pero bueno

#[tokio::test]
async fn tercero() {
    // 1. Crear pool con max 10 conexiones

    let pool = create_pool().await.expect("Failed to create pool");

    // 2. Crear 10 tareas que simulan 10 bots trabajando
    let mut handles = vec![];

    for bot_index in 0..10 {
        let pool_clone = pool.clone(); // Arc interno, barato

        let handle = tokio::spawn(async move {
            // Cada "bot" hace múltiples operaciones
            for operation in 0..5 {
                // Simular trabajo de un bot: insertar, leer, actualizar
                let mut tx = pool_clone
                    .begin()
                    .await
                    .expect(&format!("Bot {}: failed to begin tx", bot_index));

                // Operación de escritura
                sqlx::query!(
                    "INSERT INTO bots (id, name,model_name,system_prompt, openness, sociability, retention, 
                                      agreeableness, volatility, loyalty)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?,?,?)",
                    uuid::Uuid::new_v4().to_string(),
                    format!("Bot-{}-Op-{}", bot_index, operation),
                    format!("Bot-{}-Op-{}", bot_index, operation),
                    format!("Bot-{}-Op", bot_index),
                    0.5_f32,  0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32,
                )
                .execute(&mut *tx)
                .await
                .expect(&format!("Bot {}: insert failed", bot_index));

                // Operación de lectura
                let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM bots")
                    .fetch_one(&mut *tx)
                    .await
                    .expect(&format!("Bot {}: count failed", bot_index));

                tx.commit()
                    .await
                    .expect(&format!("Bot {}: commit failed", bot_index));

                println!(
                    "Bot {} completed operation {}, total bots: {}",
                    bot_index, operation, count
                );
            }

            bot_index // retornar índice para verificación
        });

        handles.push(handle);
    }

    // 3. Esperar que TODOS los bots terminen (con timeout para detectar deadlock)
    let timeout = Duration::from_secs(30);

    for handle in handles {
        let result = tokio::time::timeout(timeout, handle).await;

        match result {
            Ok(Ok(bot_index)) => {
                println!("✅ Bot {} completed successfully", bot_index);
            }
            Ok(Err(join_err)) => {
                panic!("❌ Bot task panicked: {}", join_err);
            }
            Err(_) => {
                panic!(
                    "❌ DEADLOCK DETECTED: Bot task timed out after {:?}. \
                        The pool is likely exhausted or deadlocked.",
                    timeout
                );
            }
        }
    }

    // 4. Verificar estado final
    let final_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM bots")
        .fetch_one(&pool)
        .await
        .expect("Final count failed");

    assert_eq!(final_count, 51, "Expected 50 bots (10 bots × 5 ops each)");

    println!("✅ All 10 bots completed without deadlocks!");
}
