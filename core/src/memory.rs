use crate::{
    entry::Entry,
    mode::Mode,
    path::{EntryPathRel, RootPath},
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use hashbrown::HashMap;
use id_arena::{Arena, Id};

#[derive(Debug)]
pub struct Memory {
    arena: Arena<Entry>,
    map: MemoryMap,
}

#[allow(clippy::new_without_default)]
impl Memory {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: MemoryMap {
                paths: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, entry: Entry) -> Result<()> {
        let path = entry.path.rel.clone();
        let id = self.arena.alloc(entry);

        self.map.paths.insert(path, id);
        Ok(())
    }

    #[tracing::instrument(skip(self, root))]
    pub async fn read(&mut self, mode: Mode, root: &RootPath) -> Result<()> {
        mode.read(self, root).await
    }

    #[tracing::instrument(skip(self))]
    pub fn hydrate(&mut self) -> Result<()> {
        for (_, entry) in self.arena.iter_mut() {
            entry
                .hydrate(&self.map)
                .with_context(|| format!("failed to hydrate entry {}", entry.path))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MemoryMap {
    paths: HashMap<EntryPathRel, Id<Entry>>,
}

impl MemoryMap {
    pub fn get(&self, key: &impl AsMemoryMapKey) -> Option<Id<Entry>> {
        self.paths.get(key.as_memory_map_key()).copied()
    }
}

pub trait AsMemoryMapKey {
    fn as_memory_map_key(&self) -> &Utf8Path;
}
