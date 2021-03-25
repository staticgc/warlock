mod error;
mod warlock;
mod fs;

pub use error::Error;
use keyvalue::KeyValue;
pub use warlock::Warlock;

use std::sync::Arc;

pub type Arkv = Arc<Box<dyn KeyValue>>;