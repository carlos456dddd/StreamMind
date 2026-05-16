#[derive(serde::Deserialize, Clone, Debug)]
pub struct MonitorConfig {
    pub metrics_interval_ms: u16,
    pub log_rejected_messages: bool,
    pub max_event_log_size: u16,
    pub show_token_streaming: bool,
    pub window_width: u16,
    pub window_height: u16,
    pub theme: Theme,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
    System,
}
