use serde::{Deserialize, Serialize};
use sqlite::{Connection, Statement};
use std::error;
use std::fmt;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::result;
mod config;

// constants
pub static DENDRON_CONFIG_FILE: &str = "dendron.yml";
pub static DENDRON_DB_FILE: &str = ".dendron.metadata.db";

// Errors
#[derive(Debug)]
pub enum DendronError {
    Sqlite(sqlite::Error),
    Io(io::Error)
    // Other,
    // NotFound,
}

impl fmt::Display for DendronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DendronError::Sqlite(ref err) => err.fmt(f),
            DendronError::Io(ref err) => err.fmt(f),
            // DendronError::Other => writeln!(f, "other"),
            // DendronError::NotFound => writeln!(f, "not found"),
        }
    }
}

impl error::Error for DendronError {}

impl From<io::Error> for DendronError {
    fn from(err: io::Error) -> DendronError {
        DendronError::Io(err)
    }
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

#[derive(Debug, Serialize, Deserialize)]
struct DConfigWorkspace {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DConfig {
    version: i64,
    workspace: DConfigWorkspace,
}

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

// sqlite helpers

fn create_empty_db(db_file_path: PathBuf) -> Result<Connection> {
    let connection = sqlite::open(db_file_path).map_err(DendronError::Sqlite);
    // TODO  create empty tables
    return connection;
}
