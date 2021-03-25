use std::sync::Arc;
use std::path::{PathBuf, Path};

use rusqlite::{params, Connection, NO_PARAMS};
use parking_lot::Mutex;

use crate::{Error, KeyValue};

#[derive(Clone)]
pub struct SqliteDB {
    dbpath: PathBuf,
    db: Arc<Mutex<Connection>>,
}

impl SqliteDB {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let db = Connection::open(path).map_err(|e| Error::impl_err(e.to_string()))?;

        let odb = SqliteDB {
            dbpath: path.to_owned(),
            db: Arc::new(Mutex::new(db)),
        };

        odb.init()?;
        Ok(odb)
    }

    pub fn new_box(path: &Path) -> Result<Box<dyn KeyValue>, Error> {
        let db = SqliteDB::new(path)?;
        Ok(Box::new(db))
    }

    fn init(&self) -> Result<(), Error> {
        let db = self.db.lock();
        db.execute("create table if not exists data(
            ver int,
            key text,
            value blob,
            primary key(ver, key)
        )", NO_PARAMS).map_err(|e| Error::impl_err(e.to_string()))?;

        self.begin_tx(&db)?;

        Ok(())
    }

    fn begin_tx(&self, db: &Connection) -> Result<(), Error> {
        db.execute("begin transaction", NO_PARAMS).map_err(|e| Error::impl_err(e.to_string()))?;
        Ok(())
    }

    fn end_tx(&self, db: &Connection) -> Result<(), Error> {
        db.execute("end transaction", NO_PARAMS).map_err(|e| Error::impl_err(e.to_string()))?;
        Ok(())
    }
}


impl KeyValue for SqliteDB {
    fn put(&self, ver: u32, key: &[u8], value: &[u8]) -> Result<(), Error> {

        let sql = "insert or replace into data values(?,?,?)";
        let db = self.db.lock();
        let mut stmt = db.prepare_cached(sql).map_err(|e| Error::impl_err(e.to_string()))?;

        stmt.execute(params![ver, key, value]).map_err(|e| Error::impl_err(e.to_string()))?;

        Ok(())
    }

    fn delete(&self, ver: u32, key: &[u8]) -> Result<(), Error> {
        let sql = "delete from data where ver=? and key=?";
        let db = self.db.lock();
        let mut stmt = db.prepare_cached(sql).map_err(|e| Error::impl_err(e.to_string()))?;
        stmt.execute(params![ver, key]).map_err(|e| Error::impl_err(e.to_string()))?;

        Ok(())
    }

    fn get(&self, ver: u32, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let sql = "select ver, key, value from data where ver=? and key=?";
        let db = self.db.lock();

        let mut stmt = db.prepare_cached(sql).map_err(|e| Error::impl_err(e.to_string()))?;

        let mut rows = stmt.query(params![ver, key]).map_err(|e| Error::impl_err(e.to_string()))?;
        match rows.next().map_err(|e| Error::impl_err(e.to_string()))? {
            None => {
                Ok(None)
            }
            Some(row) => {
                let buf: Vec<u8> = row.get(2).map_err(|e| Error::impl_err(e.to_string()))?;
                Ok(Some(buf))
            }
        }
    }

    fn sync(&self) -> Result<(), Error> {
        let db = self.db.lock();
        self.end_tx(&db)?;
        self.begin_tx(&db)?;
        Ok(())
    }
}