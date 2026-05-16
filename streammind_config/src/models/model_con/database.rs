use anyhow::anyhow;

use crate::models::bots::Comprobations;

#[derive(serde::Deserialize, Clone, Debug)]

// url                  = "DATABASE_URL"
// pool_size            = 10
// connect_timeout_secs = 5
// run_migrations       = true
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u16,
    pub connect_timeout_secs: u16,
    pub run_migrations: bool,
}

impl Comprobations for DatabaseConfig {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>> {
        //Comprobar que existe la variable de entorno
        //Que el poolzise no sea mayor a 10
        let mut errors = vec![];
        match dotenvy::var(&self.url) {
            Ok(_x) => {}
            Err(_c) => errors.push(anyhow!("No hay variable con nombre: {}", &self.url)),
        }

        match self.pool_size > 10 {
            true => errors.push(anyhow!("{} pasa el limite de pool_size ", &self.pool_size)),
            false => {}
        }
        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}
