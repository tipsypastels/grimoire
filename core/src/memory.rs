use crate::entry::Entry;
use camino::Utf8Path;
use hashbrown::HashMap;
use id_arena::{Arena, Id};
use std::sync::Arc;

#[derive(Debug)]
pub struct Memory {
    pub dir: Arc<Utf8Path>,
    arena: Arena<Entry>,
    paths: HashMap<Arc<Utf8Path>, Id<Entry>>,
}

impl Memory {
    pub fn new(dir: Arc<Utf8Path>) -> Self {
        Self {
            dir,
            arena: Arena::new(),
            paths: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entry: Entry) {
        let path = Arc::clone(&entry.path);
        let id = self.arena.alloc(entry);
        self.paths.insert(path, id);
    }
}
