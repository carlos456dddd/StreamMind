use anyhow;

use crate::models::bots::Comprobations;

#[derive(serde::Deserialize, Clone, Debug)]

// [youtube]
// api_key              = "REEMPLAZA_CON_TU_API_KEY"
// live_chat_id         = "REEMPLAZA_CON_TU_LIVE_CHAT_ID"
// streamer_channel_id  = "REEMPLAZA_CON_TU_CHANNEL_ID"
// poll_interval_ms     = 6000    # Milisegundos entre cada consulta al chat
//                                 # No bajar de 3000 para no gastar la cuota diaria
// max_results_per_poll = 50      # Máximo de mensajes por consulta (máx API: 2000)
// ignore_bots          = true    # Ignorar mensajes de bots de YouTube (Streamlabs, etc.)

pub struct YoutubeConfig {
    pub api_key: String,
    pub live_chat_id: String,
    pub streamer_channel_id: String,
    pub poll_interval_ms: u16,
    pub max_results_per_poll: u16,
    pub ignore_bots: bool,
}

impl Comprobations for YoutubeConfig {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>> {
        let mut _errors = vec![];
        match dotenvy::var(&self.api_key) {
            Ok(_x) => {}
            Err(f) => _errors.push(anyhow::anyhow!(
                "Problemas con la variable {}, error: {}",
                &self.api_key,
                f
            )),
        }
        match dotenvy::var(&self.live_chat_id) {
            Ok(_x) => {}
            Err(f) => _errors.push(anyhow::anyhow!(
                "Problemas con la variable {}, error: {}",
                &self.live_chat_id,
                f
            )),
        }
        match dotenvy::var(&self.streamer_channel_id) {
            Ok(_x) => {}
            Err(f) => _errors.push(anyhow::anyhow!(
                "Problemas con la variable {}, error: {}",
                &self.streamer_channel_id,
                f
            )),
        }
        match (0..2000).contains(&self.max_results_per_poll) {
            true => {}
            false => {
                _errors.push(anyhow::anyhow!(
                    "El valor {} no esta en el rango [0 - 2000]",
                    &self.max_results_per_poll
                ));
            }
        }
        if self.poll_interval_ms < 3000 {
            eprint!("Recomendación: No bajar de 3000")
        }

        match _errors.is_empty() {
            true => Ok(()),
            false => Err(_errors),
        }


    }
}
