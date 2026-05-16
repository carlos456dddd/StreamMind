use std::{env, path::Path};

use streammind_config::models::bots::{BotConfig, BotMeta, Comprobations, PersonalityConfig, SystemPromptConfig};
#[test]
fn test_1() {
    // let mut path = env::current_dir().unwrap();
    // path.push("streammind-config");
    // path.push("tests");
    // path.push("cama.toml");

    let ruta = "C:/Users/gmach/Desktop/Rust_v0/Rust_bots/StreamMind/streammind-config/tests/cama.toml";
    // Leer y cargar
    let contenido = std::fs::read_to_string(ruta).expect("No se pudo leer el archivo");
    let _config: BotConfig = toml::from_str(&contenido).expect("Error al parsear el TOML");
    let a = _config.validate().unwrap();

    let esperada = BotConfig {
        bot: BotMeta {
            name: "Aria".to_string(),
            model_name: "llama3.2:3b".to_string(),
            voice_id: "es_ES-sharvard-medium".to_string(),
            max_ctx_tokens: 4096,
        },
        personality: PersonalityConfig {
            openness: 0.00,
            sociability: 0.70,
            retention: 0.90,
            agreeableness: 0.60,
            volatility: 0.25,
            loyalty: 0.80,
        },
        system_prompt: SystemPromptConfig {
            text: "Eres Aria, una IA curiosa y entusiasta que sigue el stream de [STREAMER].\r\nTe encanta aprender cosas nuevas y compartir lo que sabes.\r\nHablas de forma amigable y directa, sin ser demasiado formal.\r\n".to_string(),
        },
    };
   let b = esperada.validate().unwrap(); // EL ERROR ESTA FUNCIONANDO, TODO BEM

    assert_eq!(a, b);
    // let a = BotConfig {
    //     bot: BotMeta {
    //         name: (),
    //         model_name: (),
    //         voice_id: (),
    //         max_ctx_tokens: (),
    //     },
    //     personality: PersonalityConfig {
    //         openness: (),
    //         sociability: (),
    //         retention: (),
    //         agreeableness: (),
    //         volatility: (),
    //         loyalty: (),
    //     },
    //     system_prompt: SystemPromptConfig { text: () },
    // };
}
