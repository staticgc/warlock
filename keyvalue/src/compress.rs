use crate::KeyValue;


use std::sync::Arc;
use crate::Error;
use log::debug;

pub struct CompressKV {
    kv: Arc<Box<dyn KeyValue>>,
}

impl CompressKV {
    pub fn new(kv: Arc<Box<dyn KeyValue>>) -> Box<dyn KeyValue> {
        Box::new(CompressKV{
            kv,
        })
    }
}

impl KeyValue for CompressKV {
    fn put(&self, ver: u32, key: &[u8], val: &[u8]) -> Result<(), Error> {
        debug!("compress put {} {} {}", ver, key.len(), val.len());

        let cbuf = zstd::block::compress(&val, 0)?;
        self.kv.put(ver, key, &cbuf)?;

        Ok(())
    }

    fn get(&self, ver: u32, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        debug!("compress get {} {}", ver, key.len());

        match self.kv.get(ver, key)? {
            None=>Ok(None),
            Some(buf) => {
                Ok(Some(zstd::block::decompress(&buf, 1024 * 1024 * 10)?))
            }
        }
    }

    fn delete(&self, ver: u32, key: &[u8]) -> Result<(), Error> {
        self.kv.delete(ver, key)?;
        Ok(())
    }

    fn sync(&self) -> Result<(), Error> {
        self.kv.sync()
    }
}

pub fn make_key(ver: u32, key: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    buf.extend_from_slice(&ver.to_be_bytes()[..]);
    buf.push(58);
    buf.extend_from_slice(key);
    buf
}