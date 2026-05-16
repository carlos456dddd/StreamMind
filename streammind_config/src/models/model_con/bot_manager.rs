use std::{env, path::Path};

use anyhow::anyhow;

use crate::models::bots::Comprobations;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct BotManagerConfig {
    pub max_bots: u16,
    pub max_bot_to_bot_turns: u16,
    pub bot_cooldown_ms: u16,
    pub response_timeout_ms: u16,
    pub bots_config_dir: String,
    pub streamer_priority: bool,
    pub broadcast_bot_messages: bool,
}

impl Comprobations for BotManagerConfig {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>> {
        let mut errors = vec![];
        match (1..11).contains(&self.max_bots) {
            true => {}
            false => errors.push(anyhow!(
                "Valor : {}, sobrepasa el limite máximo",
                self.max_bots
            )),
        }

        let _path = env::current_dir().unwrap();

        let _path_dir = env::current_dir().unwrap();
        let _path_new = _path_dir.to_str().unwrap();
        match Path::new(&[_path_new, &self.bots_config_dir].concat()).exists() {
            true => {}
            false => errors.push(anyhow!("La dirección colocada no existe")),
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}
// # -----------------------------------------------------------------------------
// # [bot_manager] — Orquestador central de bots
// # -----------------------------------------------------------------------------
// [bot_manager]
// max_bots              = 10             # Máximo de bots activos simultáneamente
// max_bot_to_bot_turns  = 3              # Turnos bot-a-bot consecutivos antes de pausar
// bot_cooldown_ms       = 3000           # Tiempo mínimo entre respuestas del mismo bot
// response_timeout_ms   = 15000         # Tiempo máximo para que un bot genere respuesta
// bots_config_dir       = "./config/bots" # Directorio con los TOML de cada bot
// streamer_priority     = true           # Los mensajes del streamer interrumpen conversaciones bot-a-bot
// broadcast_bot_messages = true          # Los mensajes de bots son visibles para los demás bots
