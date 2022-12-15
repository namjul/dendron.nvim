use crate::DENDRON_CONFIG_FILE;
use std::fs;
use std::io;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct DConfig {
    version: i64
}

impl DConfig {
    pub fn new(config_root: &PathBuf) -> io::Result<DConfig> {
        let x = config_root.join(DENDRON_CONFIG_FILE);
        println!("load config from {:#?}", x);
        return fs::read_to_string(x).map(|value| {
            let docs = YamlLoader::load_from_str(&value.to_string()).expect("Could not parse yaml file");
            let doc = &docs[0];
            DConfig {
                version: doc["version"].as_i64().unwrap()
            }
        });
    }
}
