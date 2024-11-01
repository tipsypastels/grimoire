use self::{memory::Memory, path::RootPath};
use anyhow::Result;
use camino::Utf8Path;
use std::sync::Arc;
use tokio::sync::RwLock;

mod dependency;
mod document;
mod entry;
mod memory;
mod mode;
mod path;
mod util;

pub use self::{document::*, mode::*};

#[derive(Debug, Clone)]
pub struct Grimoire {
    root: RootPath,
    mem: Arc<RwLock<Memory>>,
}

impl Grimoire {
    pub async fn new(root: impl Into<Arc<Utf8Path>>, mode: Mode) -> Result<Self> {
        let root = RootPath::new(root.into());
        let mut mem = Memory::new();

        mem.read(mode, &root).await?;
        mem.hydrate()?;

        let mem = Arc::new(RwLock::new(mem));
        Ok(Self { root, mem })
    }
}
