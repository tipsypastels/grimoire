use self::memory::*;
use anyhow::Result;
use camino::Utf8Path;
use std::sync::Arc;
use tokio::sync::RwLock;

mod dependency;
mod document;
mod entry;
mod memory;
mod mode;
mod util;

pub use self::{document::*, mode::*};

#[derive(Debug, Clone)]
pub struct Grimoire {
    dir: Arc<Utf8Path>,
    mem: Arc<RwLock<Memory>>,
}

impl Grimoire {
    pub async fn new(dir: impl Into<Arc<Utf8Path>>, mode: Mode) -> Result<Self> {
        let dir = dir.into();
        let mut mem = Memory::new(dir.clone());
        mode.read(&mut mem, &dir).await?;

        let mem = Arc::new(RwLock::new(mem));
        Ok(Self { dir, mem })
    }
}
