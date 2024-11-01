use crate::entry::Entry;
use anyhow::Result;
use camino::Utf8Path;
use hashbrown::HashMap;
use id_arena::{Arena, Id};
use std::sync::Arc;

#[derive(Debug)]
pub struct Memory {
    root: Arc<Utf8Path>,
    arena: Arena<Entry>,
    paths: HashMap<Box<Utf8Path>, Id<Entry>>,
}

impl Memory {
    pub fn new(root: Arc<Utf8Path>) -> Self {
        Self {
            root,
            arena: Arena::new(),
            paths: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entry: Entry) -> Result<()> {
        let path = Box::from(entry.rel_path(&self.root)?);
        let id = self.arena.alloc(entry);

        self.paths.insert(path, id);
        Ok(())
    }
}
