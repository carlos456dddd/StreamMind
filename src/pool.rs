use std::time::Duration;
use anyhow::{Context, Ok};
use sqlx::mysql::MySqlPoolOptions;
use dotenvy;


pub async fn create_pool() -> Result<sqlx::MySqlPool, anyhow::Error> {

    let database_url = dotenvy::var("DATABASE_URL")?;
    let b = database_url.as_str();

    let value = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::new(10, 0))
        .connect(b)
        .await
        .context("Error de conexion")?;
    sqlx::migrate!().run(&value).await?;

    Ok(value)
}
