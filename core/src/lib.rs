use self::{memory::Memory, path::RootPath};
use anyhow::Result;
use camino::Utf8Path;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::{RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};

mod dependency;
mod document;
mod memory;
mod mode;
mod node;
mod path;
mod util;

pub use self::{document::*, mode::*, node::*};

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

    pub async fn get(&self, path: impl AsRef<Utf8Path>) -> Option<Ref<'_>> {
        RwLockReadGuard::try_map(self.mem.read().await, |mem| {
            mem.get(mem.get_id(memory::TryMemoryMapKey::new(path.as_ref()))?)
        })
        .map(Ref)
        .ok()
    }

    pub async fn get_mut(&self, path: impl AsRef<Utf8Path>) -> Option<RefMut<'_>> {
        RwLockWriteGuard::try_map(self.mem.write().await, |mem| {
            mem.get_mut(mem.get_id(memory::TryMemoryMapKey::new(path.as_ref()))?)
        })
        .map(RefMut)
        .ok()
    }
}

#[derive(Debug)]
pub struct Ref<'a>(RwLockReadGuard<'a, Node>);

impl Deref for Ref<'_> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct RefMut<'a>(RwLockMappedWriteGuard<'a, Node>);

impl Deref for RefMut<'_> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RefMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
