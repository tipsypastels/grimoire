use crate::{
    mode::Mode,
    node::Node,
    path::{NodePathRel, RootPath},
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use hashbrown::HashMap;
use id_arena::{Arena, Id};

#[derive(Debug)]
pub struct Memory {
    arena: Arena<Node>,
    map: MemoryMap,
}

#[allow(clippy::new_without_default)]
impl Memory {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: MemoryMap {
                paths: HashMap::new(),
            },
        }
    }

    pub fn get(&self, id: Id<Node>) -> Option<&Node> {
        self.arena.get(id)
    }

    pub fn get_mut(&mut self, id: Id<Node>) -> Option<&mut Node> {
        self.arena.get_mut(id)
    }

    pub fn get_id<K>(&self, key: &K) -> Option<Id<Node>>
    where
        K: AsMemoryMapKey + ?Sized,
    {
        self.map.get(key)
    }

    pub fn insert(&mut self, node: Node) -> Result<()> {
        let path = node.path.rel.clone();
        let id = self.arena.alloc(node);

        self.map.paths.insert(path, id);
        Ok(())
    }

    #[tracing::instrument(skip(self, root))]
    pub async fn read(&mut self, mode: Mode, root: &RootPath) -> Result<()> {
        mode.read(self, root).await
    }

    #[tracing::instrument(skip(self))]
    pub fn hydrate(&mut self) -> Result<()> {
        for (_, node) in self.arena.iter_mut() {
            node.hydrate(&self.map)
                .with_context(|| format!("failed to hydrate node {}", node.path))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MemoryMap {
    paths: HashMap<NodePathRel, Id<Node>>,
}

impl MemoryMap {
    pub fn get<K>(&self, key: &K) -> Option<Id<Node>>
    where
        K: AsMemoryMapKey + ?Sized,
    {
        self.paths.get(key.as_memory_map_key()).copied()
    }
}

pub trait AsMemoryMapKey {
    fn as_memory_map_key(&self) -> &Utf8Path;
}

#[repr(transparent)]
pub struct TryMemoryMapKey(Utf8Path);

impl TryMemoryMapKey {
    pub fn new(path: &Utf8Path) -> &Self {
        unsafe { std::mem::transmute(path) }
    }
}

impl AsMemoryMapKey for TryMemoryMapKey {
    fn as_memory_map_key(&self) -> &Utf8Path {
        &self.0
    }
}
