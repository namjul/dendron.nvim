mod config;

use config::DConfig;
use std::path::Path;

// constants
pub static DENDRON_CONFIG_FILE: &str = "dendron.yml";
pub static DENDRON_DB_FILE: &str = ".dendron.metadata.db";

fn main() {
    let ws_root_opt =
        home::home_dir().map(|home_dir| home_dir.join(Path::new("./Dropbox/dendron")));

    ws_root_opt.map(|ws_root| {
        let d_config = DConfig::new(&ws_root);

        println!("{:#?}", d_config.unwrap());
    });
}
