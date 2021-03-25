use anyhow::Error;
use std::{path::Path};
use std::sync::Arc;

use warlock::{Warlock, Arkv};

fn init_kv(dbpath: &Path) -> Result<Arkv, Error> {
    let db = keyvalue::sqlite::SqliteDB::new_box(dbpath)?;

    let db = Arc::new(db);

    Ok(db)
}

fn init_test() -> Result<(), Error> {
    let db = init_kv(&Path::new("./test.db"))?;

    let w = Warlock::new(db);
    w.init()?;

    Ok(())
}

fn main() -> Result<(), Error> {
    init_test()?;

    Ok(())
}
