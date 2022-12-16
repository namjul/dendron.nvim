use sqlite::Connection;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::result;
use yaml_rust::YamlLoader;

// constants
pub static DENDRON_CONFIG_FILE: &str = "dendron.yml";
pub static DENDRON_DB_FILE: &str = ".dendron.metadata.db";

// Errors
#[derive(Debug)]
enum DendronError {
    Sqlite(sqlite::Error),
    Other,
}

impl From<sqlite::Error> for DendronError {
    fn from(err: sqlite::Error) -> DendronError {
        DendronError::Sqlite(err)
    }
}

type Result<T> = result::Result<T, DendronError>;

// traits
trait Disposable {
    fn dispose() -> ();
}
trait DataStore<V> {
    // fn get<K>(key: K) -> Result<V>;
    // fn find(opts: dyn Any) -> Result<Vec<V>>;
    // fn write<K>(key: K, data: V) -> Result<K>;
    // fn delete<K>(key: K) -> Result<String>;
    fn query(&self, query_string: Option<String>) -> Result<Vec<V>>;
}

#[derive(Debug)]
struct DNoteLink {
    // type: "ref" | "wiki" | "md" | "backlink" | "linkCandidate" | "frontmatterTag";
    // position?: Position;
    // from: DNoteLoc;
    // to?: DNoteLoc;
    // data: TData;
}

#[derive(Debug)]
struct NotePropsMeta {
    fname: String,
    links: Vec<DNoteLink>,
    // anchors: { [index: string]: DNoteAnchorPositioned | undefined };
    // type: DNodeType;
    // stub?: boolean;
    // schemaStub?: boolean;
    // parent: DNodePointer | null;
    // children: DNodePointer[];
    // data: T;
    // body: string;
    // custom?: TCustom;
    // schema?: { moduleId: string; schemaId: string };
    // vault: DVault;
    // contentHash?: string;
    // color?: string;
    // tags?: string | string[];
    // image?: DNodeImage;
    // traits?: string[];
}

struct SqliteMetaDataStore {
    connection: Connection,
}

impl DataStore<NotePropsMeta> for SqliteMetaDataStore {
    fn query(&self, query_string: Option<String>) -> Result<Vec<NotePropsMeta>> {
        if query_string.is_none() {
            let sql = format!("SELECT * FROM NoteProps WHERE fname = '{}'", "root");
            self.connection.execute(sql)?
        }

        // self.connection.execute
        Ok(vec![])
    }
}

impl SqliteMetaDataStore {
    fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

trait FileStore {}

#[derive(Debug)]
pub struct DConfig {
    version: i64,
}

impl DConfig {
    pub fn new(config_root: &PathBuf) -> io::Result<DConfig> {
        let x = config_root.join(DENDRON_CONFIG_FILE);
        println!("load config from {:#?}", x);
        return fs::read_to_string(x).map(|value| {
            let docs =
                YamlLoader::load_from_str(&value.to_string()).expect("Could not parse yaml file");
            let doc = &docs[0];
            DConfig {
                version: doc["version"].as_i64().unwrap(),
            }
        });
    }
}

fn main() {
    let ws_root =
        match home::home_dir().map(|home_dir| home_dir.join(Path::new("./Dropbox/dendron"))) {
            Some(value) => value,
            None => panic!("No workspace root found"),
        };

    let d_config = DConfig::new(&ws_root);

    let db_file_path = ws_root.join(DENDRON_DB_FILE);

    let connection = create_empty_db(db_file_path).expect("Could not create empty db");

    let sqlite_meta_data_store = SqliteMetaDataStore::new(connection);

    let result = sqlite_meta_data_store.query(None);

    println!("{:#?}", result);
    println!("{:#?}", d_config.unwrap().version);
}

// sqlite helpers

fn create_empty_db(db_file_path: PathBuf) -> Result<Connection> {
    let connection = sqlite::open(db_file_path).map_err(DendronError::Sqlite);
    // TODO  create empty tables
    return connection
}

