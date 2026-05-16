use anyhow::anyhow;

use crate::models::bots::Comprobations;

#[derive(serde::Deserialize, Clone, Debug)]

// [ollama]
// host                 = "http://localhost:11434" #Env
// default_model        = "llama3.2:3b"   # Modelo usado si el bot no especifica uno
// request_timeout_secs = 60              # Timeout por request de generación
// max_tokens           = 300             # Máximo de tokens a generar por respuesta
// temperature          = 0.8             # Creatividad: 0.0 = determinista, 1.0 = caótico
// keep_alive           = "10m"           # Cuánto mantiene Ollama el modelo en VRAM

pub struct OllamaConfig {
    pub host: String,
    pub default_model: String,
    pub request_timeout_secs: u32,
    pub max_tokens: u16,
    pub temperature: f32,
    pub keep_alive: String,
}

impl Comprobations for OllamaConfig {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>> {
        let mut _error = vec![];
        match dotenvy::var(&self.host) {
            Ok(_x) => {}
            Err(f) => _error.push(anyhow!(
                "Problemas con la variable{}, error: {}",
                &self.host,
                f
            )),
        }
        let _range_temperatura = 0.0..1.0;
        match _range_temperatura.contains(&self.temperature) {
            true => {}
            false => {
                _error.push(anyhow!(
                    "El valor {}, no esta en el rango [0.0 - 1.0]",
                    &self.temperature
                ));
            }
        }
        match _error.is_empty() {
            true => Ok(()),
            false => Err(_error),
        }
    }
}
