use std::env;

use streammind_config::models::config::{AppConfig, ComprobationConfig};

#[test]
fn test_3() {
    let _direct = env::current_dir().unwrap();
    let _sum_direct = _direct.join("config").join("default.toml");
    let binding = std::fs::read_to_string(&_sum_direct).unwrap();
    let news = binding.as_str();
    let tom: AppConfig = toml::from_str(news).unwrap();
    let gh = tom.validate();
    
    match gh {
        Ok(d) => {
            println!("Todo ben")
        }
        Err(d) => {
            println!("Algo mal {:?}", d)
        }
    }

    assert_eq!(5, 2)
}
