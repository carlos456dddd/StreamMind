#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WhisperModel {
    Tiny(String),
    Base,
    Small,
    Medium,
    Large,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct SttConfig {
    pub enabled: bool,
    pub whisper_model: WhisperModel,
    pub language: String,
    pub silence_threshold_ms: u16,
    pub min_speech_ms: u16,
    pub vad_enabled: bool,
}

// impl Comprobations for TtsConfig {
//     fn validate(&self) -> Result<(), Vec<anyhow::Error>> {}
// }
// # -----------------------------------------------------------------------------
// # [stt] — Reconocimiento de voz del streamer (Speech-to-Text)
// # Convierte tu voz en texto para que los bots puedan responderte.
// #
// # whisper_model puede ser: "tiny" | "base" | "small" | "medium" | "large"
// #   "tiny"    ~75 MB  — muy rápido, poca precisión
// #   "base"    ~145 MB — buen balance para español
// #   "small"   ~465 MB — recomendado si tu CPU lo aguanta
// #   "medium"  ~1.5 GB — alta precisión, requiere buen hardware
// #
// # Los modelos se descargan automáticamente en el primer uso desde:
// #   https://huggingface.co/ggerganov/whisper.cpp
// # -----------------------------------------------------------------------------
// [stt]
// enabled              = true
// whisper_model        = "base"
// language             = "es"            # Código de idioma para el reconocimiento
// silence_threshold_ms = 1200            # Milisegundos de silencio para detectar fin de frase
// min_speech_ms        = 300             # Ignorar sonidos menores a este tiempo (evita falsos positivos)
// vad_enabled          = true            # Voice Activity Detection — reduce el procesamiento innecesario
