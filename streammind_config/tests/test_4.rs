
use std::env;
use streammind_config::{loader, sync};
use streammind_db::pool;

#[tokio::test]
async fn test_4() {
    let _pool = pool::create_pool().await.unwrap();

    // let botis = sqlx::query("SELECT * FROM bots")
    //     .fetch_all(&_pool)
    //     .await
    //     .unwrap();

    // let _cama: Vec<Bot> = botis
    //     .into_iter()
    //     .map(|x| Bot::from_row(&x).unwrap())
    //     .collect();

    // let mut ga = vec![];

    // for fuerza_g in bots {
    //     ga.push(Bot::from_row(&fuerza_g).unwrap());
    // }

    // println!("Miremos el error en mi cara: {:?}", _cama);

    let g = env::current_dir().unwrap();
    let d = g.join("config").join("bots");

    let bots = loader::load_all_bot_configs(&d).unwrap();


   let asf = sync::sync_bots_to_db(&_pool, &bots).await;
  
    
    println!("Quiero ver cambios {:?}", asf.unwrap());

    assert_eq!(5,8);



    // //Parchao , me parcharon pipipipipi ia de mrd, se cree pendeja algún día seré mas rapido, aunque igual su respuesta al inicio fue una mrd jaaaaaaaaaaa


    // //Limpiar para ejemplo todo trunco si mi correspondiente a la base de datos -> Bot[CARLOS, MELANI, SIMON] y lo que tengo para añadir es BotConfig[MELANI, SIMON] tengo que tener los que son iguales
    // //RPT1[MELANI, SIMON]  junto RPT2[1, 2] => Su struct literal y su indice correspondiente al Vec<Bot[]>

    // let mut_bot: Vec<Bot> = botis
    //     .into_iter()
    //     .map(|x| Bot::from_row(&x).unwrap())
    //     .collect();

    // // let mut bad = HashMap::new();

    // let bots_por_nombre: HashMap<&String, &Bot> = mut_bot.iter().map(|b| (&b.name, b)).collect();

    // let _recorrido_bot_config: HashSet<&String> = bots.iter().map(|x| &x.bot.name).collect();
    // // _recorido.iter().filter(|x| {

    // // }).collect();

    // //Interesante la función partition() para usar en dos escenarios los que cumplen y los que no :O hasta creí que tenía que sar esa maldita tontera de for_each()
    // let (_respuesta1, _respuesta2): (Vec<&BotConfig>, Vec<&BotConfig>) = bots
    //     .iter()
    //     .partition(|x| !bots_por_nombre.contains_key(&x.bot.name));

    // let _quien_es: Vec<&String> = _respuesta2.iter().map(|x| &x.bot.name).collect();

    // //No estan - Están
    // // println!("Veamos estan: {:?}, tenemos el 2: {:?} \n", _respuesta1, _respuesta2);

    // // let _final_clash: Vec<Option<&Bot>> = _recorido
    // //     .into_iter()
    // //     .enumerate()
    // //     .map(|(y, l)| -> Option<&Bot> {
    // //         if _quien_es.contains(&l) {
    // //             Some(&mut_bot[y])
    // //         } else {
    // //             None
    // //         }
    // //     })
    // //     .collect();
    // let _final_clash: Vec<&Bot> = _quien_es
    //     .iter()
    //     .filter_map(|nombre| {
    //         // Buscamos en el mapa. Si existe, devuelve Some(&Bot), si no, None.
    //         // filter_map se encarga de eliminar los None automáticamente.
    //         bots_por_nombre.get(nombre).copied()
    //     })
    //     .collect();

    // // let lista_sin_some : Vec<&Bot> = _final_clash.into_iter().filter_map(|x|x).collect();
    // // Los datos pero del lado del botdb[Los que se repiten]
    // println!(
    //     "Tenemos supuestamente los que se repiten, en db del otro lado: {:?}",
    //     _final_clash
    // );

    // println!(
    //     "Respuesta del lado del cambio de configuración: {:?}",
    //     _respuesta2
    // );

    // let respuesta_n =
    // _recorido
    // .iter()
    // .enumerate()
    // .map(|(x,s)| {
    //     _recorido.contains(&_respuesta2[x].bot.name)

    // })
    // .collect();

    // mut_bot.iter().enumerate().for_each(|(d, f)| {
    //     for (x, y) in _configs.iter().enumerate() {
    //         if y.bot.name == f.name {
    //             bad.insert(x, (d, y));
    //             break;
    //         }
    //     }
    // });

    // let mut good = HashMap::new();

    // _configs.iter().enumerate().for_each(|(_x, _m)| {
    //     good.insert(_x, _m);
    // });

    // for j in &bad {
    //     good.remove(j.0);
    // }

    // _respuesta2.iter().filter(|x| _recorido.contains(&x.bot.name))
    // let mut gg = vec![];
    // _final_clash.iter().for_each(|x|

    //     match x {
    //         Some(c) => gg.push(c),
    //         None => {}

    //     }
    // );

    // gg.into_iter().enumerate().for_each(|(x,y)| {

    //     println!("BOT_CONFIG:[{:?}], BOT[{:?}]", _respuesta2[x], y);
    //     // let _send_bot = Bot {
    //     //     id: mut_bot[u].id,
    //     //     name: mut_bot[u].name,
    //     //     voice_id: mut_bot[u].voice_id,
    //     //     model_name: mut_bot[u].model_name,
    //     //     system_prompt: mut_bot[u].system_prompt,
    //     //     personality_stats: PersonalityStats {
    //     //         openness: mut_bot[u].personality_stats.openness,
    //     //         sociability: mut_bot[u].personality_stats.sociability,
    //     //         retention: mut_bot[u].personality_stats.retention,
    //     //         agreeableness: mut_bot[u].personality_stats.agreeableness,
    //     //         volatility: mut_bot[u].personality_stats.volatility,
    //     //         loyalty: mut_bot[u].personality_stats.loyalty,
    //     //     },
    //     //     max_ctx_tokens: mut_bot[u].max_ctx_tokens,
    //     //     is_active: mut_bot[u].is_active,
    //     //     created_at: mut_bot[u].created_at,
    //     // };
    //     // match x {
    //     //     Some(a) => ,
    //     //     None =>,

    //     // }

    //     // _opti_bots.push(bot_to_bot_for_changes(_respuesta2[x], &mut_bot[y.unwrap()]));
    // });

    // println!("La ia pega duro: {:?}", _values_la_ia_pega_fuerte);

    // let mut esa = vec![];

    // bots.iter().for_each(|x| {
    //     for (g, f) in _cama.iter().enumerate() {
    //         if x.bot.name == f.name {
    //             esa.push(g);
    //             break;
    //         }
    //     }
    // });

    // let mut vector_result = vec![];
    // _cama.into_iter().enumerate().for_each(|(d,n)| {if !esa.contains(&d) && n.is_active == Some(true) {vector_result.push(n.id);} });

    // println!("Esperemos que si, pero creo que sí: {:?}", vector_result);

    // let date_result_negative: Vec<Option<String>> = _cama.iter().map(|d| {   if !esa.contains(&&d) && d.is_active == Some(true) {Some(*d.id)} else {Option::None}}).collect();
    // println!("Datos que no estan en la base de datos: {:?}", date_result_negative);

    // let h: HashMap<u32, &BotConfig> = bots.iter().collect();
    // for k in _cama {}

    // let esacosa = sync::sync_bots_to_db(&_pool, bots).await.unwrap();
    // println!("Veamos el valor de retorno es : {:?}", esacosa);
    
}
