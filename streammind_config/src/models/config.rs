use anyhow;

use crate::models::{
    bots::Comprobations,
    model_con::{bot_manager, database, log, monitor, ollama, stream, stt, tts, youtube},
};

pub trait ComprobationConfig {
    fn validate(&self) -> Result<(), Vec<(ErrorPath, Vec<anyhow::Error>)>>;
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct AppConfig {
    pub stream: stream::StreamConfig,
    pub database: database::DatabaseConfig,
    pub ollama: ollama::OllamaConfig,
    pub youtube: youtube::YoutubeConfig,
    pub tts: tts::TtsConfig,
    pub stt: stt::SttConfig,
    pub bot_manager: bot_manager::BotManagerConfig,
    pub monitor: monitor::MonitorConfig,
    pub log: log::LogConfig,
}

#[derive(Debug)]
pub enum ErrorPath {
    Stream,
    Database,
    Ollama,
    Youtube,
    Tts,
    Stt,
    BotManager,
    Monitor,
    Log,
}

impl ComprobationConfig for AppConfig {
    fn validate(&self) -> Result<(), Vec<(ErrorPath, Vec<anyhow::Error>)>> {
        let mut errors_two = vec![];
        match self.database.validate() {
            Ok(_x) => {}
            Err(d) => errors_two.push((ErrorPath::Database, d)),
        }

        match self.ollama.validate() {
            Ok(_x) => {}
            Err(d) => errors_two.push((ErrorPath::Ollama, d)),
        }

        match self.youtube.validate() {
            Ok(_x) => {}
            Err(d) => errors_two.push((ErrorPath::Youtube, d)),
        }

        match self.tts.validate() {
            Ok(_x) => {}
            Err(d) => errors_two.push((ErrorPath::Tts, d)),
        }
        match self.bot_manager.validate() {
            Ok(_x) => {}
            Err(d) => errors_two.push((ErrorPath::BotManager, d)),
        }

        match errors_two.is_empty() {
            true => Ok(()),
            false => Err(errors_two),
        }
    }
}
