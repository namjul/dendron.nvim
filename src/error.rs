use std::error;
use std::fmt;
use std::io;

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
