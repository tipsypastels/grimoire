use self::storage::Storage;
use anyhow::Result;
use camino::Utf8Path;
use std::sync::Arc;
use tokio::sync::RwLock;

mod document;
mod storage;

#[derive(Debug)]
pub struct Grimoire {
    dir: Arc<Utf8Path>,
    storage: Arc<RwLock<Storage>>,
}

impl Grimoire {
    pub fn builder(dir: impl Into<Arc<Utf8Path>>) -> GrimoireBuilder {
        GrimoireBuilder {
            dir: dir.into(),
            storage: Storage::default(),
        }
    }
}

#[derive(Debug)]
pub struct GrimoireBuilder {
    dir: Arc<Utf8Path>,
    storage: Storage,
}

impl GrimoireBuilder {
    pub async fn walk(mut self) -> Self {
        self.storage.walk(&self.dir).await;
        self
    }

    pub async fn walk_and_read(mut self) -> Result<Self> {
        self.storage.walk_and_read(&self.dir).await?;
        Ok(self)
    }

    pub fn build(self) -> Grimoire {
        Grimoire {
            dir: self.dir,
            storage: Arc::new(RwLock::new(self.storage)),
        }
    }
}
