use std::{env, path::PathBuf};

use anyhow::anyhow;

use crate::models::bots::Comprobations;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct TtsConfig {
    pub engine: String,
    pub piper_binary: String,
    pub piper_models_dir: String,
    pub elevenlabs_api_key: String,
    pub audio_output_device: String,
    pub audio_queue_timeout_ms: u16,
}

//     engine               = "piper"
// piper_binary         = "/usr/local/bin/piper"     # Ruta al ejecutable de Piper
// piper_models_dir     = "./models/piper"            # Directorio con archivos .onnx y .json
// elevenlabs_api_key   = ""                          # Solo si engine = "elevenlabs"
// audio_output_device  = "default"                   # "default" usa el dispositivo del sistema
// audio_queue_timeout_ms = 30000

impl Comprobations for TtsConfig {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>> {
        let mut errors = vec![];

        let g = env::current_dir().unwrap();

        //No se puede hacer demasiado por el momento pero lo dejare así para que pase la prueva
        match PathBuf::from([g.to_str().unwrap(), &self.piper_binary.as_str()].concat()).exists() {
            true => {}
            false => errors.push(anyhow!("La dirección de {}, no existe ", self.piper_binary)),
        }
        match PathBuf::from([g.to_str().unwrap(), &self.piper_models_dir.as_str()].concat())
            .exists()
        {
            true => {}
            false => errors.push(anyhow!(
                "La dirección de {}, no existe ",
                self.piper_models_dir
            )),
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }

        /* OJO para los dispositivos del sistema creo que seran mas importantes despues, sobre todo para usarlos en la parte de default que se dice */
    }
}
