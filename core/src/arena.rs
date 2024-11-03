use crate::{
    node::Node,
    path::{NodePathRel, RootPath},
    util,
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use futures::StreamExt;
use hashbrown::HashMap;

pub type Id = id_arena::Id<Node>;

#[derive(Debug, Default)]
pub struct Arena {
    inner: id_arena::Arena<Node>,
    paths: ArenaPaths,
}

impl Arena {
    pub fn get(&self, id: Id) -> Option<&Node> {
        self.inner.get(id)
    }

    pub fn get_id<P: ArenaPath + ?Sized>(&self, path: &P) -> Option<Id> {
        self.paths.get(path)
    }

    pub fn insert(&mut self, node: Node) {
        let path = node.path.rel.clone();
        let id = self.inner.alloc(node);
        self.paths.map.insert(path, id);
    }

    #[tracing::instrument(skip_all)]
    pub async fn load_all(&mut self, root: &RootPath) -> Result<()> {
        let mut stream = util::walk_dir_and_read(root).await;
        while let Some(result) = stream.next().await {
            let (path, ref text) = result?;
            if let Some(node) = Node::new(root.clone(), path.into(), text).await? {
                self.insert(node);
            };
        }
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn hydrate_all(&self) -> Result<()> {
        for (_, node) in self.inner.iter() {
            node.hydrate(&self.paths)
                .with_context(|| format!("failed to hydrate {}", node.path))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ArenaPaths {
    map: HashMap<NodePathRel, Id>,
}

impl ArenaPaths {
    pub fn get<P: ArenaPath + ?Sized>(&self, path: &P) -> Option<Id> {
        self.map.get(path.arena_path()).copied()
    }
}

pub trait ArenaPath {
    fn arena_path(&self) -> &Utf8Path;
}

#[repr(transparent)]
pub struct AsArenaPath(Utf8Path);

impl AsArenaPath {
    pub fn new(path: &Utf8Path) -> &Self {
        unsafe { std::mem::transmute(path) }
    }
}

impl ArenaPath for AsArenaPath {
    fn arena_path(&self) -> &Utf8Path {
        &self.0
    }
}
