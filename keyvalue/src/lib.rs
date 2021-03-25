mod error;
pub mod sqlite;
pub mod rocksdb;
pub mod compress;

pub use error::{Error, ErrorKind};


pub trait KeyValue {
    fn put(&self, ver: u32, key: &[u8], val: &[u8]) -> Result<(), Error>;
    fn get(&self, ver: u32, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    fn delete(&self, ver: u32, key: &[u8]) -> Result<(), Error>;

    fn put_str(&self, ver: u32, key: &str, val: &[u8]) -> Result<(), Error> {
        self.put(ver, key.as_bytes(), val)
    }

    fn get_str(&self, ver: u32, key: &str) -> Result<Option<Vec<u8>>, Error> {
        self.get(ver, key.as_bytes())
    }

    fn sync(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub fn make_key(ver: u32, key: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    buf.extend_from_slice(&ver.to_be_bytes()[..]);
    buf.push(58);
    buf.extend_from_slice(key);
    buf
}