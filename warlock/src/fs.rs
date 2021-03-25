use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use parking_lot::RwLock;

use keyvalue::KeyValue;
use crate::{Error, Arkv};


pub struct FileSystem {
    kv: Arkv,
    state: Arc<RwLock<FState>>,
} 

impl FileSystem {
    pub fn init(kv: Arkv) -> Result<(), Error> {
        let mut state = FState::default();
        FileSystem::init_root(&state, &kv)?;

        state.put(&kv)?;

        Ok(())
    }

    fn init_root(st: &FState, kv: &Arkv) -> Result<(), Error> {
        let entries: Vec<DirEntry> = Vec::new();
        let buf = rmp_serde::to_vec_named(&entries)?;
        kv.put_str(st.cver, "n/", buf.as_ref())?;

        Ok(())
    }
}


#[derive(Serialize, Deserialize)]
pub struct FState {
    ino: AtomicU64,
    cver: u32,
    minver: u32,
    maxver: u32,
}

impl Default for FState {
    fn default() -> Self {
         FState {
             ino: AtomicU64::new(10),
             cver: 1,
             minver: 0,
             maxver: 0,
         }
    }
}

impl FState {
    fn gen_ino(&self) -> u64 {
        self.ino.fetch_add(1, Ordering::Relaxed)
    }

    fn put(&mut self, kv: &Arkv) -> Result<(), Error> {
        let buf = rmp_serde::to_vec_named(&self)?;
        kv.put_str(0, "fs-state", buf.as_ref())?;

        Ok(())
    }

    fn get(kv: &Arkv) -> Result<FState, Error> {
        let buf = kv.get_str(0, "fs-state")?
            .ok_or(Error::unknown_str("fs-state not found"))?;
       
        let state: FState = rmp_serde::from_slice(buf.as_ref())?; 

        Ok(state)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub etype: u8,
    pub ino: u64,
    pub ver: u32,
}