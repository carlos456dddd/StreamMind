use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use streammind_config::{
    loader,
    models::bots::{BotConfig, Comprobations},
};
#[test]
fn test_path() {
    let path = "/streammind_config";
    let g = env::current_dir().unwrap();
    let a = PathBuf::from("\\cama");
    let d = g.join("config").join("bots");

    if let Ok(entradas) = std::fs::read_dir(&d) {
        for entrada in entradas {
            if let Ok(archivo) = entrada {
                // Imprime la ruta completa de cada archivo
                println!("{:?}", archivo.path());
            }
        }
    }
    let mut camas_vector: Vec<PathBuf> = vec![];
    std::fs::read_dir(&d).unwrap().into_iter().for_each(|x| {
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

    let f = loader::load_all_bot_configs(&d).unwrap();

    println!("Tenemos lo siguiente: {:?}", f);

    // println!(
    //     "Tenemos lo siguiente para el lado de la situación : {:?}",
    //     _cama_all_situation
    // );

    // for cama in &camas_vector {
    //     let fg = fs::read_to_string(cama).expect("camapower");

    //     println!("Tenemos que : {}", fg);
    // }

    // println!("Tenemos un vector con las cosas {:?}", camas_vector);

    let b = Path::new("/sdaasd");

    assert_eq!(2, 8);
    // loader::load_all_bot_configs("");
}
