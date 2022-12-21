use dendron_nvim::constants::DENDRON_DB_FILE;
use dendron_nvim::config;
use std::path::Path;
use dendron_nvim::create_empty_db;
use dendron_nvim::SqliteMetaDataStore;
use dendron_nvim::DataStore;

fn main() {
    let ws_root =
        match home::home_dir().map(|home_dir| home_dir.join(Path::new("./Dropbox/dendron"))) {
            Some(value) => value,
            None => panic!("No workspace root found"),
        };

    let d_config = config::DConfig::new(&ws_root);

    let db_file_path = ws_root.join(DENDRON_DB_FILE);

    let connection = create_empty_db(db_file_path).expect("Could not create empty db");

    let sqlite_meta_data_store = SqliteMetaDataStore::new(connection);

    let result = sqlite_meta_data_store.query(None);

    println!("{:#?}", result);
    println!("{:#?}", d_config);
}
