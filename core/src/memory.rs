use crate::entry::Entry;
use anyhow::{Context, Result};
use camino::Utf8Path;
use hashbrown::HashMap;
use id_arena::{Arena, Id};
use std::sync::Arc;

#[derive(Debug)]
pub struct Memory {
    arena: Arena<Entry>,
    map: MemoryMap,
}

impl Memory {
    pub fn new(root: Arc<Utf8Path>) -> Self {
        Self {
            arena: Arena::new(),
            map: MemoryMap {
                root,
                paths: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, entry: Entry) -> Result<()> {
        let path = Box::from(entry.rel_path(&self.map.root)?);
        let id = self.arena.alloc(entry);

        self.map.paths.insert(path, id);
        Ok(())
    }

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
    root: Arc<Utf8Path>,
    paths: HashMap<Box<Utf8Path>, Id<Entry>>,
}

impl MemoryMap {
    pub fn id(&self, path: impl AsRef<Utf8Path>) -> Option<Id<Entry>> {
        self.paths.get(path.as_ref()).copied()
    }

    pub fn id_abs(&self, path: impl AsRef<Utf8Path>) -> Option<Id<Entry>> {
        self.id(path.as_ref().strip_prefix(self.root.as_ref()).ok()?)
    }
}
