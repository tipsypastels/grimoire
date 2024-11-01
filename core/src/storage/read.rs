use super::{entry::Entry, util, Storage};
use anyhow::Result;
use camino::Utf8Path;
use futures::{stream::FuturesUnordered, StreamExt};
use std::pin::pin;

#[allow(async_fn_in_trait)]
pub trait Read {
    async fn read(self, storage: &mut Storage, dir: &Utf8Path) -> Result<()>;
}

pub struct Eager;

impl Read for Eager {
    async fn read(self, storage: &mut Storage, dir: &Utf8Path) -> Result<()> {
        let mut futures = pin!(util::walk_dir(dir))
            .map(|path| async move { Entry::new_with_content(path.into()).await })
            .collect::<FuturesUnordered<_>>()
            .await;
        while let Some(entry) = futures.next().await {
            storage.insert(entry?);
        }
        Ok(())
    }
}

pub struct Shallow;

impl Read for Shallow {
    async fn read(self, storage: &mut Storage, dir: &Utf8Path) -> Result<()> {
        let mut stream = pin!(util::walk_dir(dir));
        while let Some(path) = stream.next().await {
            storage.insert(Entry::new(path.into()));
        }
        Ok(())
    }
}

pub struct Lazy;

impl Read for Lazy {
    async fn read(self, _storage: &mut Storage, _dir: &Utf8Path) -> Result<()> {
        Ok(())
    }
}
