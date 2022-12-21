use sqlite::{Connection, Statement};
use std::path::PathBuf;
use std::result;
pub mod error;
pub mod constants;
pub mod config;

type Result<T> = result::Result<T, error::DendronError>;

trait Disposable {
    fn dispose() -> ();
}

pub trait DataStore<V> {
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
pub struct NotePropsMeta {
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

pub struct SqliteMetaDataStore {
    connection: Connection,
}

impl SqliteMetaDataStore {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

impl DataStore<NotePropsMeta> for SqliteMetaDataStore {
    fn query(&self, query_string: Option<String>) -> Result<Vec<NotePropsMeta>> {
        if query_string.is_none() {
            let sql = format!("SELECT * FROM NoteProps WHERE fname = '{}'", "root");
            self.connection.execute(sql)?
        }

        Ok(vec![])
    }
}

struct NotePropsDbContext<'a> {
    pub get_by_id: Option<Statement<'a>>,
}

impl<'a> NotePropsDbContext<'a> {
    fn new() -> Self {
        return NotePropsDbContext { get_by_id: None };
    }
    fn create_table(connection: &Connection) -> Result<()> {
        return connection
            .execute(
                "
                CREATE TABLE IF NOT EXISTS NoteProps (
                    id TEXT NOT NULL PRIMARY KEY,
                    fname TEXT NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT,
                    updated INTEGER,
                    created INTEGER,
                    anchors TEXT,
                    stub BOOLEAN,
                    custom TEXT,
                    contentHash TEXT,
                    color TEXT,
                    image TEXT,
                    traits TEXT
                );
                CREATE INDEX IF NOT EXISTS idx_NoteProps_fname ON NoteProps (fname)
                ",
            )
            .map_err(error::DendronError::Sqlite);
    }
    // fn get_by_id(&self, connection: &Connection, id: &str) -> Result<()> {
    //     if let None = &self.get_by_id {
    //         let stmt = connection.prepare("SELECT * FROM NoteProps WHERE id = :id")?;
    //         self.get_by_id = Some(stmt);
    //     };
    //     let result = self
    //         .get_by_id
    //         .as_mut()
    //         .unwrap()
    //         .into_iter()
    //         .bind((":id", id))
    //         .map(|row| row.unwrap());
    //         .map_err(DendronError::Sqlite)
    //
    //     return result;
    // }
}

trait FileStore {}

trait Engine {}

// sqlite helpers

pub fn create_empty_db(db_file_path: PathBuf) -> Result<Connection> {
    let connection = sqlite::open(db_file_path).map_err(error::DendronError::Sqlite)?;

    // let mut noteprops_db_context = NotePropsDbContext::new();

    NotePropsDbContext::create_table(&connection)?;

    // TODO  create empty tables
    return Ok(connection);
}
