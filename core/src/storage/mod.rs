use self::entry::Entry;
use anyhow::Result;
use camino::Utf8Path;
use futures::{stream::FuturesUnordered, StreamExt};
use hashbrown::HashMap;
use id_arena::{Arena, Id};
use std::{pin::pin, sync::Arc};

mod entry;
mod util;

#[derive(Debug, Default)]
pub struct Storage {
    arena: Arena<Entry>,
    paths: HashMap<Arc<Utf8Path>, Id<Entry>>,
}

impl Storage {
    pub async fn walk(&mut self, dir: &Utf8Path) {
        let mut stream = pin!(util::walk_dir(dir));
        while let Some(path) = stream.next().await {
            self.insert(Entry::new(path.into()));
        }
    }

    pub async fn walk_and_read(&mut self, dir: &Utf8Path) -> Result<()> {
        let mut futures = pin!(util::walk_dir(dir))
            .map(|path| async move { Entry::new_with_content(path.into()).await })
            .collect::<FuturesUnordered<_>>()
            .await;
        while let Some(entry) = futures.next().await {
            self.insert(entry?);
        }
        Ok(())
    }

    pub fn insert(&mut self, entry: Entry) {
        let path = Arc::clone(&entry.path);
        let id = self.arena.alloc(entry);
        self.paths.insert(path, id);
    }
}
