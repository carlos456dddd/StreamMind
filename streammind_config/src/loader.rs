use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Error;

use crate::models::{
    bots::{BotConfig, Comprobations},
    config::{AppConfig, ComprobationConfig, ErrorPath},
};

pub fn load_all_bot_configs(dir: &Path) -> Result<Vec<BotConfig>, anyhow::Error> {
    //config/bots/*.toml
    //Necesito todos los documentos de //config/bots/*.toml para que al final obtenga los resultados de carga en un vector
    //Es necesario realizar la validación en este caso la implementación que se uso anteriormente
    let mut camas_vector: Vec<PathBuf> = vec![];
    std::fs::read_dir(dir).unwrap().into_iter().for_each(|x| {
        camas_vector.push(x.unwrap().path());
    });



    let _cama_all_situation: Vec<BotConfig> = camas_vector
        .iter()
        .map(|x| {
            let cama = std::fs::read_to_string(&x).expect("error");
            return toml::from_str(cama.as_str()).unwrap();
        })
        .collect();

    _cama_all_situation.iter().for_each(|x| {
        x.validate().unwrap();
    });

    Ok(_cama_all_situation)
}

pub fn load_app_config() -> Result<AppConfig, Vec<(ErrorPath, Vec<Error>)>> {
    //Tenemos que leer el toml, y sacar la info para esa webada
    let _direct = env::current_dir().unwrap();
    let _sum_direct = _direct.join("config").join("default.toml");
    
    let binding = std::fs::read_to_string(&_sum_direct).unwrap();
    let news = binding.as_str();

    let tom: AppConfig = toml::from_str(news).unwrap();
    let gh = tom.validate();

    match gh {
        Ok(()) => Ok(tom),
        Err(d) => Err(d),
    }
}
