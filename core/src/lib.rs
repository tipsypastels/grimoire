use self::{arena::Arena, path::RootPath};
use anyhow::Result;
use camino::Utf8Path;
use futures::{stream, StreamExt};
use std::{ops::Deref, sync::Arc};
use tokio::sync::{RwLock, RwLockReadGuard as RG};

mod arena;
mod dependency;
mod document;
mod node;
mod path;
mod util;

pub use self::{dependency::*, document::*, node::*};

#[derive(Debug, Clone)]
pub struct Grimoire {
    root: RootPath,
    arena: Arc<RwLock<Arena>>,
}

impl Grimoire {
    pub async fn new(root: Arc<Utf8Path>) -> Result<Self> {
        let root = RootPath::new(root);
        let mut arena = Arena::default();

        arena.load_all(&root).await?;
        arena.hydrate_all()?;

        let arena = Arc::new(RwLock::new(arena));
        Ok(Self { root, arena })
    }

    pub async fn get(&self, path: impl AsRef<Utf8Path>) -> Option<Page<'_>> {
        RG::try_map(self.arena.read().await, |arena| {
            arena.get(arena.get_id(arena::AsArenaPath::new(path.as_ref()))?)
        })
        .map(Page)
        .ok()
    }

    pub async fn deps(&self, node: &Node) -> Option<Dependencies<Page<'_>>> {
        let deps = node.deps.load();
        let deps = deps.as_deref()?;
        let deps = stream::iter(deps.iter())
            .filter_map(|dep| async move {
                let arena = self.arena.read().await;
                let node = RG::try_map(arena, |arena| arena.get(dep.id())).ok()?;
                Some((node.path.rel.clone(), Page(node)))
            })
            .collect()
            .await;

        Some(deps)
    }
}

#[derive(Debug)]
pub struct Page<'a>(RG<'a, Node>);

impl Deref for Page<'_> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// pub struct DependenciesPage<'a>(HashMap<NodePathRel, Page<'a>>);

// impl<'a> Deref for DependenciesPage<'a> {
//     type Target = HashMap<NodePathRel, Page<'a>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
