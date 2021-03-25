
use keyvalue::KeyValue;
use std::sync::Arc;
use crate::{Error, Arkv};
use crate::fs::FileSystem;

pub struct Warlock {
    kv: Arkv,
}

impl Warlock {
    pub fn new(kv: Arkv) -> Self {
        Warlock {
            kv,
        }
    }

    pub fn init(&self) -> Result<(), Error> {
        if self.kv.get_str(0, "init")?.is_some() {
            return Ok(())
        }

        FileSystem::init(self.kv.clone())?;

        self.kv.put_str(0, "init", b"ok")?;
        self.kv.sync()?;

        Ok(())
    }
}