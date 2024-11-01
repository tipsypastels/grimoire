use crate::{entry::Entry, memory::Memory, util};
use anyhow::Result;
use camino::Utf8Path;
use futures::{stream::FuturesUnordered, StreamExt};
use std::pin::pin;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    WalkAndRead,
    Walk,
    Noop,
}

impl Mode {
    pub async fn read(self, mem: &mut Memory, dir: &Utf8Path) -> Result<()> {
        match self {
            Self::WalkAndRead => mode_walk_and_read(mem, dir).await,
            Self::Walk => mode_walk(mem, dir).await,
            Self::Noop => Ok(()),
        }
    }
}

async fn mode_walk_and_read(mem: &mut Memory, dir: &Utf8Path) -> Result<()> {
    let mut futures = pin!(util::walk_dir(dir))
        .map(|path| async move {
            let text = util::read_to_string(&path).await?;
            Entry::new(path.into(), Some(text.into()))
        })
        .collect::<FuturesUnordered<_>>()
        .await;
    while let Some(entry) = futures.next().await {
        mem.insert(entry?);
    }
    Ok(())
}

async fn mode_walk(mem: &mut Memory, dir: &Utf8Path) -> Result<()> {
    let mut stream = pin!(util::walk_dir(dir));
    while let Some(path) = stream.next().await {
        mem.insert(Entry::new(path.into(), None)?);
    }
    Ok(())
}
