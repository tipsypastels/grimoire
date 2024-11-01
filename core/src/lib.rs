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
    root: Arc<Utf8Path>,
    mem: Arc<RwLock<Memory>>,
}

impl Grimoire {
    pub async fn new(root: impl Into<Arc<Utf8Path>>, mode: Mode) -> Result<Self> {
        let root = root.into();
        let mut mem = Memory::new(root.clone());
        mode.read(&mut mem, &root).await?;

        let mem = Arc::new(RwLock::new(mem));
        Ok(Self { root, mem })
    }
}
