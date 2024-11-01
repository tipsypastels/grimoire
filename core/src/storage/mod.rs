use self::entry::Entry;
use camino::Utf8Path;
use hashbrown::HashMap;
use id_arena::{Arena, Id};
use std::sync::Arc;

mod entry;
pub mod read;
mod util;

#[derive(Debug, Default)]
pub struct Storage {
    arena: Arena<Entry>,
    paths: HashMap<Arc<Utf8Path>, Id<Entry>>,
}

impl Storage {
    pub fn insert(&mut self, entry: Entry) {
        let path = Arc::clone(&entry.path);
        let id = self.arena.alloc(entry);
        self.paths.insert(path, id);
    }
}
