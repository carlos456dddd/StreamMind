#[derive(serde::Deserialize, Clone, Debug)]
pub struct StreamConfig {
    pub streamer_name: String,
    pub stream_title: String,
    pub game_or_topic: String,
    pub schedule: Vec<String>,
    pub timezone: String
}

