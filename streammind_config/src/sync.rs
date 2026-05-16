use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use chrono::Timelike;
use sqlx::{
    FromRow,
    types::uuid,
};
use streammind_db::{
    self,
    models::bots::{Bot, PersonalityStats},
    queries::get_bot_by_id,
};

use crate::models::bots::BotConfig;

// sync_bots_to_db(pool, configs: Vec<BotConfig>) -> Result<Vec<Bot>>
#[derive(Debug)]
pub struct MandiBot<'a> {
    pub id: &'a str,
    pub personality_stats: Option<PersonalityStats>,
    pub system_prompt: Option<&'a str>,
}

pub struct BotSend<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub voice_id: &'a str,
    pub model_name: &'a str,
    pub system_prompt: &'a str,
    pub personality_stats: [Option<f32>; 5],
    pub max_ctx_tokens: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub fn bot_to_bot_for_changes<'a>(y: &'a BotConfig, d: &'a Bot) -> MandiBot<'a> {
    MandiBot {
        id: &d.id,
        personality_stats: Some(PersonalityStats {
            openness: match y.personality.openness != d.personality_stats.openness.unwrap() {
                true => Some(y.personality.openness),
                false => None,
            },
            sociability: match y.personality.sociability != d.personality_stats.sociability.unwrap()
            {
                true => Some(y.personality.sociability),
                false => None,
            },
            retention: match y.personality.retention != d.personality_stats.retention.unwrap() {
                true => Some(y.personality.retention),
                false => None,
            },
            agreeableness: match y.personality.agreeableness
                != d.personality_stats.agreeableness.unwrap()
            {
                true => Some(y.personality.agreeableness),
                false => None,
            },
            loyalty: match y.personality.loyalty != d.personality_stats.loyalty.unwrap() {
                true => Some(y.personality.loyalty),
                false => None,
            },
            volatility: match y.personality.volatility != d.personality_stats.volatility.unwrap() {
                true => Some(y.personality.volatility),
                false => None,
            },
        }),
        system_prompt: match y.system_prompt.text != d.system_prompt {
            true => Some(&y.system_prompt.text),
            false => None,
        },
    }
}

pub async fn sync_bots_to_db<'a>(
    _pool: &sqlx::MySqlPool,
    _configs: &'a Vec<BotConfig>,
) -> Result<Vec<Bot>, Vec<anyhow::Error>> {
    //Haré una busqueda por nombres en el bot config, si no existe lo inserta y devuelve
    //Si existe actualiza los stats de personalidad y sistem prompt si cambiaron
    //Si en el toml no esta lo marca como inactivo

    // let mut _send_bots = vec![]; // -> Bot -> A la base de datos
    let mut _opti_bots = vec![]; // -> Caso de sets, cambios -> Retornar los ID de los que se cambiaron y llamarlos
    let mut _errores = vec![];

    let a = sqlx::query("SELECT * FROM bots")
        .fetch_all(_pool)
        .await
        .unwrap();

    let mut_bot: Vec<Bot> = a.into_iter().map(|x| Bot::from_row(&x).unwrap()).collect();

    let bots_por_nombre: HashMap<&String, &Bot> = mut_bot.iter().map(|b| (&b.name, b)).collect();

    // En este caso se tiene los datos de `Incluidos en BD` `Nuevos en BD`

    let (_respuesta1, _respuesta2): (Vec<&BotConfig>, Vec<&BotConfig>) = _configs
        .iter()
        .partition(|x| !bots_por_nombre.contains_key(&x.bot.name));

    //Caballo ganador//println!("Quiero ver los datos de respuesta 1: {:?}", &_respuesta2);

    let _quien_es: Vec<&String> = _respuesta2.iter().map(|x| &x.bot.name).collect();

    let _final_clash: Vec<&Bot> = _quien_es
        .into_iter()
        .filter_map(|nombre| bots_por_nombre.get(nombre).copied())
        .collect();

    // Datos de los que `no se estan usando en la BD`

    let _names_config: HashSet<&String> = _configs.iter().map(|x| &x.bot.name).collect();

    let _values_la_ia_pega_fuerte: Vec<&String> = mut_bot
        .iter()
        .filter(|x| x.is_active == Some(true) && !_names_config.contains(&x.name))
        .map(|y| &y.id)
        .collect();

    _final_clash.into_iter().enumerate().for_each(|(x, y)| {
        _opti_bots.push(bot_to_bot_for_changes(_respuesta2[x], &y));
    });

    //Lógica para poner en false `los que no se estén usando`

    let _names_config: HashSet<&String> = _configs.iter().map(|x| &x.bot.name).collect();

    let _values_la_ia_pega_fuerte: Vec<&String> = mut_bot
        .iter()
        .filter(|x| x.is_active == Some(true) && !_names_config.contains(&x.name))
        .map(|y| &y.id)
        .collect();

    // println!("Conjunto de bots para poner en desactivados: {:?}", &_values_la_ia_pega_fuerte);{
    for i in _values_la_ia_pega_fuerte {
        let a = sqlx::query("UPDATE bots SET is_active = false WHERE id = ?")
            .bind(i)
            .execute(_pool)
            .await;
        match a {
            Ok(_) => () ,
            Err(e) => _errores.push(anyhow!("Error insercion: {:?} | Tipo: {:?}", &i, e))
        }
    }

    //Podría hacer una implementación pata que sea mas sencillo

    //Para control de errores, sería mejor usar una función un contro de errores un result y con anyhow :)
    let botijas: Vec<Bot> = _respuesta1
        .into_iter()
        .map(|f| Bot {
            id: uuid::Uuid::new_v4().into(),
            name: f.bot.name.to_string(),
            voice_id: Some(f.bot.voice_id.to_string()),
            model_name: f.bot.model_name.to_string(),
            system_prompt: f.system_prompt.text.to_string(),
            personality_stats: PersonalityStats {
                openness: Some(f.personality.openness),
                sociability: Some(f.personality.sociability),
                retention: Some(f.personality.retention),
                agreeableness: Some(f.personality.agreeableness),
                volatility: Some(f.personality.volatility),
                loyalty: Some(f.personality.loyalty),
            },
            max_ctx_tokens: Some(f.bot.max_ctx_tokens.try_into().unwrap_or(0)),
            is_active: Some(true),
            created_at: Some(chrono::Utc::now().naive_utc().with_nanosecond(0).unwrap()),
        })
        .collect();

    //Ahora tendría que usar la wea fome esa para realizar una insersión multiple

    let mut tx = _pool.begin().await.unwrap();

    let mut bots_b = vec![];

    for a in botijas {
        sqlx::query!(
            "
    INSERT INTO bots(id, name, voice_id, model_name, system_prompt, openness, sociability, retention, agreeableness, volatility, loyalty, max_ctx_tokens, is_active, created_at ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        a.id,
        a.name,
        a.voice_id,
        a.model_name,
        a.system_prompt,
        a.personality_stats.openness,
        a.personality_stats.sociability,
        a.personality_stats.retention,
        a.personality_stats.agreeableness,
        a.personality_stats.volatility,
        a.personality_stats.loyalty,
        a.max_ctx_tokens,
        a.is_active,
        a.created_at
        )
        .execute(&mut *tx)
        .await.unwrap();

        bots_b.push(a);
    }

    match tx.commit().await {
        Ok(_a) => {}
        Err(b) => {
            _errores.push(anyhow!("Error de insercion, tipo {:?}", b));
        }
    };

    for f in _opti_bots {
        let ge = f.personality_stats.unwrap();
        let mut send_values = vec![];
        // let mut send_string = vec![];
        // send_string.push(("id", f.id));

        match ge.agreeableness {
            Some(a) => {
                send_values.push(("agreeableness", a));
            }
            None => {}
        }

        match ge.loyalty {
            Some(a) => {
                send_values.push(("loyalty", a));
            }
            None => {}
        }
        match ge.openness {
            Some(a) => {
                send_values.push(("openness", a));
            }
            None => {}
        }
        match ge.retention {
            Some(a) => {
                send_values.push(("retention", a));
            }
            None => {}
        }
        match ge.sociability {
            Some(a) => {
                send_values.push(("sociability", a));
            }
            None => {}
        }
        match ge.volatility {
            Some(a) => {
                send_values.push(("volatility", a));
            }
            None => {}
        }

        let _prompt = match &f.system_prompt {
            Some(_) => true,
            None => false,
        };

        let (_keys, _values): (Vec<&str>, Vec<f32>) = send_values.into_iter().unzip();

        let change_key: Vec<String> = _keys.into_iter().map(|f| format!("{} = ?", f)).collect();

        let keys = if change_key.len() > 0 {
            Some(change_key.join(", "))
        } else {
            None
        };

        let _query = match keys {
            Some(g) => Some(match &f.system_prompt {
                Some(_a) => {
                    format!(
                        "UPDATE bots SET {}, system_prompt = ? WHERE id = '{}'",
                        g, f.id
                    )
                }
                None => format!("UPDATE bots SET {} WHERE id = '{}'", g, f.id),
            }),
            None => None,
        };

        match _query {
            Some(j) => {
                let cama = j.as_str();

                let mut sql_query = sqlx::query(cama);

                for v in _values {
                    println!("Estoy usando : {:?}", &v);
                    sql_query = sql_query.bind(v);
                }
                if _prompt {
                    sql_query = sql_query.bind(f.system_prompt);
                }

                let operate = sql_query.execute(_pool).await;

                match operate {
                    Ok(_) => {
                        let g = get_bot_by_id(_pool, &f.id).await;
                        bots_b.push(g);
                    }
                    Err(f) => {
                        _errores.push(anyhow!("Error en insercion tipo: {}", f));
                    }
                }
            }
            None => {}
        }
    }

    Ok(bots_b)
}
