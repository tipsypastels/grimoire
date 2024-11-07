use crate::db::DbNode;
use anyhow::Result;
use camino::Utf8Path;

mod kind;
mod path;

pub use kind::*;
pub use path::*;

#[derive(Debug)]
pub struct Node {
    pub(crate) id: i64,
    path: NodePath,
    data: NodeData,
}

impl Node {
    pub fn path(&self) -> &NodePath {
        &self.path
    }

    pub fn name(&self) -> &str {
        self.data.name()
    }

    pub fn kind(&self) -> NodeKind {
        self.data.kind()
    }

    pub fn data(&self) -> &NodeData {
        &self.data
    }

    pub(crate) fn revive(root: &Utf8Path, node: DbNode) -> Result<Self> {
        let path = NodePath::revive(root, node.path);
        let kind = node.kind.parse::<NodeKind>()?;
        let data = kind.create(&path, &node.text)?;

        Ok(Self {
            id: node.id,
            path,
            data,
        })
    }
}

#[derive(Debug)]
pub struct NodeHead {
    pub(crate) id: i64,
    path: NodePath,
    name: Box<str>,
    kind: NodeKind,
}

impl NodeHead {
    pub fn path(&self) -> &NodePath {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub(crate) fn revive(root: &Utf8Path, node: DbNode) -> Result<Self> {
        let path = NodePath::revive(root, node.path);
        let name = node.name;
        let kind = node.kind.parse::<NodeKind>()?;

        Ok(Self {
            id: node.id,
            path,
            name,
            kind,
        })
    }
}

#[derive(Debug)]
pub(crate) struct NewNode<'a> {
    pub path: &'a NodePath,
    pub data: &'a NodeData,
}
