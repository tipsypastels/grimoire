use self::{index::Index, path::RootPath};
use anyhow::{Context, Result};
use camino::Utf8Path;
use futures::StreamExt;
use id_arena::Arena;
use std::sync::Arc;

mod dependency;
mod document;
mod index;
mod node;
mod path;
mod util;

pub use self::{dependency::*, document::*, node::*};

#[derive(Debug)]
pub struct Grimoire {
    root: RootPath,
    arena: Arena<Node>,
    index: Index,
}

impl Grimoire {
    pub fn new(root: Arc<Utf8Path>) -> Self {
        let root = RootPath::new(root);
        let arena = Arena::default();
        let index = Index::default();
        Self { root, arena, index }
    }

    pub fn get(&self, path: impl AsRef<Utf8Path>) -> Option<&Node> {
        let path = index::AsIndexPath::new(path.as_ref());
        let id = self.index.get(path)?;
        self.arena.get(id)
    }

    pub fn deps(&self, node: &Node) -> Option<Dependencies<'_>> {
        let deps = node.deps.load();
        let deps = deps.as_deref()?;
        let deps = deps.iter().filter_map(|dep| {
            let node = self.arena.get(dep.id())?;
            Some((node.path.rel.clone(), node))
        });

        Some(Dependencies(deps.collect()))
    }

    pub fn nodes(&self) -> Nodes<'_> {
        Nodes(self.arena.iter())
    }

    pub fn insert(&mut self, node: Node) {
        let path = node.path.rel.clone();
        let id = self.arena.alloc(node);
        self.index.insert(path, id);
    }

    #[tracing::instrument(skip(self))]
    pub async fn populate(&mut self) -> Result<()> {
        let mut stream = util::walk_dir_and_read(&self.root).await;
        while let Some(result) = stream.next().await {
            let (path, ref text) = result?;
            if let Some(node) = Node::new(self.root.clone(), path.into(), text).await? {
                self.insert(node);
            };
        }
        Ok(())
    }

    // Doesn't actually need &mut, but more semantically correct.
    #[tracing::instrument(skip_all)]
    pub fn hydrate(&mut self) -> Result<()> {
        for (_, node) in self.arena.iter() {
            node.hydrate(&self.index)
                .with_context(|| format!("failed to hydrate {}", node.path))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Nodes<'a>(id_arena::Iter<'a, Node, id_arena::DefaultArenaBehavior<Node>>);

impl<'a> Iterator for Nodes<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_id, node)| node)
    }
}
