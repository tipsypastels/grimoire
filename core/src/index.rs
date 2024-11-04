use crate::{node::NodeId, path::NodePathRel};
use camino::Utf8Path;
use hashbrown::HashMap;
use std::mem;

#[derive(Debug, Default)]
pub struct Index {
    map: HashMap<NodePathRel, NodeId>,
}

impl Index {
    pub fn get<P: IndexPath + ?Sized>(&self, path: &P) -> Option<NodeId> {
        self.map.get(path.index_path()).copied()
    }

    pub fn insert(&mut self, path: NodePathRel, id: NodeId) {
        self.map.insert(path, id);
    }
}

pub trait IndexPath {
    fn index_path(&self) -> &Utf8Path;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AsIndexPath(Utf8Path);

impl AsIndexPath {
    pub fn new(path: &Utf8Path) -> &Self {
        unsafe { mem::transmute(path) }
    }
}

impl IndexPath for AsIndexPath {
    fn index_path(&self) -> &Utf8Path {
        &self.0
    }
}
