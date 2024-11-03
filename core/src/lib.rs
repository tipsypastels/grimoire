use self::{arena::Arena, path::RootPath};
use anyhow::Result;
use camino::Utf8Path;
use std::{ops::Deref, sync::Arc};
use tokio::sync::{RwLock, RwLockReadGuard};

mod arena;
mod dependency;
mod document;
mod node;
mod path;
mod util;

pub use self::{document::*, node::*};

#[derive(Debug, Clone)]
pub struct Grimoire {
    root: RootPath,
    arena: Arc<RwLock<Arena>>,
}

impl Grimoire {
    pub async fn new(root: Arc<Utf8Path>) -> Result<Self> {
        let root = RootPath::new(root);
        let mut arena = Arena::default();

        arena.load_all(&root).await?;
        arena.hydrate_all()?;

        let arena = Arc::new(RwLock::new(arena));
        Ok(Self { root, arena })
    }

    pub async fn get(&self, path: impl AsRef<Utf8Path>) -> Option<Page<'_>> {
        RwLockReadGuard::try_map(self.arena.read().await, |arena| {
            arena.get(arena.get_id(arena::AsArenaPath::new(path.as_ref()))?)
        })
        .map(Page)
        .ok()
    }
}

#[derive(Debug)]
pub struct Page<'a>(RwLockReadGuard<'a, Node>);

impl Deref for Page<'_> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
