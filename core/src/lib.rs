use self::storage::Storage;
use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

mod document;
mod storage;

pub use storage::read::{self, Read};

#[derive(Debug)]
pub struct Grimoire {
    dir: Arc<Utf8Path>,
    storage: Arc<RwLock<Storage>>,
}

impl Grimoire {
    pub async fn new(dir: Utf8PathBuf, read: impl Read) -> Result<Self> {
        let dir: Arc<Utf8Path> = dir.into();

        let mut storage = Storage::default();
        read.read(&mut storage, &dir).await?;

        let storage = Arc::new(RwLock::new(storage));

        Ok(dbg!(Self { dir, storage }))
    }
}
