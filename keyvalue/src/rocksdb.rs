
use std::sync::Arc;
use std::path::Path;
use rocksdb::DB;

use crate::{KeyValue, make_key, Error};

pub struct RocksDB {
    db: Arc<DB>,
}

impl RocksDB {
    pub fn new(dirpath: &Path) -> Result<Self, Error> {
        let db = DB::open_default(dirpath)
            .map_err(|e| Error::impl_err(e.to_string()))?;

        Ok(RocksDB{
            db: Arc::new(db),
        })
    }

    pub fn new_box(dirpath: &Path) -> Result<Box<dyn KeyValue>, Error> {
        let db = Box::new(RocksDB::new(dirpath)?);

        Ok(db)
    }
}

impl KeyValue for RocksDB {
    fn put(&self, ver: u32, key: &[u8], val: &[u8]) -> Result<(), Error> {
        let real_key = make_key(ver, key);

        self.db.put(&real_key, &val)
            .map_err(|e| Error::impl_err(e.to_string()))?;

        Ok(())
    }

    fn get(&self, ver: u32, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let real_key = make_key(ver, key);

        let buf = self.db.get(&real_key).map_err(|e| Error::impl_err(e.to_string()))?;

        Ok(buf)
    }

    fn delete(&self, ver: u32, key: &[u8]) -> Result<(), Error> {
        let real_key = make_key(ver, key);
        self.db.delete(&real_key).map_err(|e| Error::impl_err(e.to_string()))?;
        Ok(())
    }
}