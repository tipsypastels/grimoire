use crate::{entry::Entry, memory::Memory, path::RootPath, util};
use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use std::pin::pin;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    WalkAndRead,
    Walk,
    Noop,
}

impl Mode {
    pub async fn read(self, mem: &mut Memory, root: &RootPath) -> Result<()> {
        match self {
            Self::WalkAndRead => mode_walk_and_read(mem, root).await,
            Self::Walk => mode_walk(mem, root).await,
            Self::Noop => Ok(()),
        }
    }
}

async fn mode_walk_and_read(mem: &mut Memory, root: &RootPath) -> Result<()> {
    let mut futures = pin!(util::walk_dir(root))
        .map(|path| async move {
            let text = util::read_to_string(&path).await?;
            anyhow::Ok((path, text))
        })
        .collect::<FuturesUnordered<_>>()
        .await;
    while let Some(result) = futures.next().await {
        let (path, text) = result?;
        let entry = Entry::new(root.clone(), path.into(), Some(text.into()))?;
        mem.insert(entry)?;
    }
    Ok(())
}

async fn mode_walk(mem: &mut Memory, root: &RootPath) -> Result<()> {
    let mut stream = pin!(util::walk_dir(root));
    while let Some(path) = stream.next().await {
        let entry = Entry::new(root.clone(), path.into(), None)?;
        mem.insert(entry)?;
    }
    Ok(())
}
