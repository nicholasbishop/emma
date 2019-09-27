use crate::buffer::{Buffer, BufferId, BufferKind, BufferMap};
use std::{error, fmt, fs, io, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    SqliteError(rusqlite::Error),
    HomeDirNotFound,
    InvalidBufferKind,
    CreateDirFailed(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::SqliteError(e)
    }
}

fn open_db() -> Result<rusqlite::Connection, Error> {
    let dir = dirs::home_dir()
        .ok_or(Error::HomeDirNotFound)?
        .join(".cache/emma");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(Error::CreateDirFailed)?;
    }
    let path = dir.join("state.sqlite3");
    let conn = rusqlite::Connection::open(&path)?;
    Ok(conn)
}

pub fn init_db() -> Result<(), Error> {
    let conn = open_db()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS open_buffers (
                      buffer_id TEXT PRIMARY KEY,
                      path TEXT,
                      kind TEXT)",
        rusqlite::NO_PARAMS,
    )?;
    Ok(())
}

pub fn load_buffer_list() -> Result<BufferMap, Error> {
    let conn = open_db()?;
    let mut stmt =
        conn.prepare("SELECT buffer_id, path, kind FROM open_buffers")?;

    let open_buffers = stmt
        .query_and_then(rusqlite::NO_PARAMS, |b| {
            let id: String = b.get(0)?;
            let path: String = b.get(1)?;
            let kind: String = b.get(2)?;
            let id = BufferId::from_str(&id).unwrap();
            Ok((
                id.clone(),
                Buffer {
                    id,
                    path: PathBuf::from(path),
                    kind: BufferKind::from_str(&kind)
                        .ok_or(Error::InvalidBufferKind)?,
                    text: None,
                },
            ))
        })?
        .collect::<Result<BufferMap, Error>>()?;
    Ok(open_buffers)
}

pub fn add_buffer(buffer: &Buffer) -> Result<(), Error> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO open_buffers (buffer_id, path, kind)
                  VALUES (?1, ?2, ?3)",
        &[
            buffer.id.to_string().as_str(),
            buffer.path.to_str().unwrap(),
            buffer.kind.to_str(),
        ],
    )?;
    Ok(())
}
