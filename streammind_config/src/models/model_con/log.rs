use std::path::PathBuf;

//"trace" | "debug" | "info" | "warn" | "error"
#[derive(serde::Deserialize, Clone, Debug)]
pub struct LogConfig {
    pub level: Level,
    pub log_to_file: bool,
    pub log_dir: PathBuf, //Asegurarse que se esta usando los correctos datos para cada estructura, cosa que creo que realmente no
    pub pretty_print: bool,
    pub filter: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
