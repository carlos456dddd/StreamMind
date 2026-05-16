use anyhow::{Error, anyhow};

pub trait Comprobations {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>>;
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct BotConfig {
    pub bot: BotMeta,
    pub personality: PersonalityConfig,
    pub system_prompt: SystemPromptConfig,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct BotMeta {
    pub name: String,
    pub model_name: String,
    pub voice_id: String,
    pub max_ctx_tokens: u32,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct PersonalityConfig {
    pub openness: f32,
    pub sociability: f32,
    pub retention: f32,
    pub agreeableness: f32,
    pub volatility: f32,
    pub loyalty: f32,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct SystemPromptConfig {
    pub text: String,
}
impl Comprobations for BotConfig {
    fn validate(&self) -> Result<(), Vec<anyhow::Error>> {
        //Necesitamos poder mandar los errores
        let mut _errores: Vec<Error> = vec![];

        //Determinamos los rangos si existen
        let _range_personality_config = 0.0..1.0;
        let _range_max_tokens = 512..32768;

        // let hash = HashMap::new();
        //Valores que estamso usando
        let _vec_tor = vec![
            self.personality.openness,
            self.personality.sociability,
            self.personality.retention,
            self.personality.agreeableness,
            self.personality.volatility,
            self.personality.loyalty,
        ];

        let _vecor_result = _vec_tor
            .iter()
            .find(|x| _range_personality_config.contains(*x) == false);

        let _vect_map: Vec<bool> = _vec_tor
            .iter()
            .map(|x| {
                if _range_personality_config.contains(x) == false {
                    false
                } else {
                    true
                }
            })
            .collect();
        _vect_map.iter().enumerate().for_each(|(x, y)| {
            if *y {
            } else {
                match x {
                    0 => _errores.push(anyhow!(
                        "Bot {}: personality.openness = {} está fuera del rango [0.0, 1.0]",
                        &self.bot.name,
                        _vec_tor[x]
                    )),
                    1 => _errores.push(anyhow!(
                        "Bot {}: personality.sociability = {} está fuera del rango [0.0, 1.0]",
                        &self.bot.name,
                        _vec_tor[x]
                    )),
                    2 => _errores.push(anyhow!(
                        "Bot {}: personality.retention = {} está fuera del rango [0.0, 1.0]",
                        &self.bot.name,
                        _vec_tor[x]
                    )),
                    3 => _errores.push(anyhow!(
                        "Bot {}: personality.agreeableness = {} está fuera del rango [0.0, 1.0]",
                        &self.bot.name,
                        _vec_tor[x]
                    )),
                    4 => _errores.push(anyhow!(
                        "Bot {}: personality.volatility = {} está fuera del rango [0.0, 1.0]",
                        &self.bot.name,
                        _vec_tor[x]
                    )),
                    5 => _errores.push(anyhow!(
                        "Bot {}: personality.loyalty = {} está fuera del rango [0.0, 1.0]",
                        &self.bot.name,
                        _vec_tor[x]
                    )),
                    6_usize.. => todo!(),
                }
            }
        });

        if (&self.bot.model_name).is_empty() {
            _errores.push(anyhow!("bot.model_name is empty"));
        };

        if _range_max_tokens.contains(&self.bot.max_ctx_tokens) == false {
            _errores.push(anyhow!(
                "bot.max_ctx_tokens = {} está fuera del rango [512 , 32768]",
                &self.bot.max_ctx_tokens
            ))
        };

        if _errores.len() == 0 {
            Ok(())
        } else {
            Err(_errores)
        }

        // let vec_tor = vec![
        //     self.openness,
        //     self.sociability,
        //     self.retention,
        //     self.agreeableness,
        //     self.volatility,
        //     self.loyalty,
        // ];
        // let range = 0.0..1.0;
        // let vecor_result = vec_tor.iter().find(|x| range.contains(*x) == false);
        // match vecor_result {
        //     Some(x) => Err(anyhow!(" {} esta fuera del rango", x)),
        //     None => Ok(PersonalityConfig {
        //         openness: self.openness,
        //         sociability: self.sociability,
        //         retention: self.retention,
        //         agreeableness: self.agreeableness,
        //         volatility: self.volatility,
        //         loyalty: self.loyalty,
        //     }),
        // }
    }
}
// impl Comprobations for BotMeta {
//     fn validate(&self) -> Result<BotMeta, anyhow::Error> {
//         let _range = 512..32768;
//         let _name = (&self.model_name).is_empty();
//         let _range_value = _range.contains(&self.max_ctx_tokens);
//         if _name == false {
//             return Err(anyhow!("Empty model name"));
//         } else if _range_value == false {
//             return Err(anyhow!("{} Esta fuera del rango", &self.max_ctx_tokens));
//         } else {
//             Ok(BotMeta {
//                 name: self.name.clone(),
//                 model_name: self.model_name.clone(),
//                 voice_id: self.voice_id.clone(),
//                 max_ctx_tokens: self.max_ctx_tokens,
//             })
//         }
//     }
// }
