use crate::{
    arena::ArenaPaths,
    dependency::DependencyRef,
    document::Document,
    path::{NodePath, RootPath},
};
use anyhow::{Context, Result};
use arc_swap::{ArcSwap, ArcSwapOption};
use camino::Utf8Path;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct Node {
    pub path: NodePath,
    name: ArcSwap<Arc<str>>,
    #[serde(skip_serializing)]
    deps: ArcSwapOption<Arc<[DependencyRef]>>,
    kind: NodeDataKind,
}

impl Node {
    pub async fn new(root: RootPath, path: Box<Utf8Path>, text: &str) -> Result<Option<Self>> {
        let path = NodePath::new(root, path)?;
        let Some(kind) = NodeDataKind::determine(path.extension()) else {
            return Ok(None);
        };

        let data = kind.create(&path, text)?;
        let name = data.name();
        let deps = data.deps();

        tracing::debug!(%name, %path, "node");

        let name = ArcSwap::from_pointee(name);
        let deps = ArcSwapOption::from_pointee(deps);

        Ok(Some(Self {
            path,
            name,
            deps,
            kind,
        }))
    }

    pub fn hydrate(&self, path_map: &ArenaPaths) -> Result<()> {
        let deps = self.deps.load();
        let Some(deps) = deps.as_deref() else {
            return Ok(());
        };
        for dep in deps.as_ref() {
            dep.hydrate(&self.path, path_map)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct NodeContent {
    pub text: Box<str>,
    pub data: NodeData,
}

#[derive(Debug, Serialize)]
pub enum NodeData {
    Document(Document),
}

impl NodeDataTrait for NodeData {
    fn name(&self) -> Arc<str> {
        match self {
            Self::Document(d) => d.name(),
        }
    }

    fn deps(&self) -> Option<Arc<[DependencyRef]>> {
        match self {
            Self::Document(d) => d.deps(),
        }
    }
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum NodeDataKind {
    Document,
}

impl NodeDataKind {
    fn create(self, path: &NodePath, text: &str) -> Result<NodeData> {
        macro_rules! match_kind {
            ($(($kind:ident, $tag:literal)),*$(,)?) => {
                match self {
                    $(
                        Self::$kind => Ok(NodeData::$kind($kind::new(path, text).with_context(|| format!(concat!("failed to create ", $tag, " {}"), path))?))
                    )*
                }
            };

        }

        match_kind! {
            (Document, "document"),
        }
    }

    fn determine(extension: Option<&str>) -> Option<Self> {
        match extension {
            Some("md") => Some(Self::Document),
            _ => None,
        }
    }
}

pub(crate) trait NodeDataTrait {
    fn name(&self) -> Arc<str>;
    fn deps(&self) -> Option<Arc<[DependencyRef]>>;
}
