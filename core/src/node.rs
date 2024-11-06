use crate::{
    dependency::Dependency,
    document::Document,
    index::Index,
    path::{NodePath, RootPath},
    util,
};
use anyhow::{Context, Result};
use arc_swap::{ArcSwap, ArcSwapOption};
use camino::Utf8Path;
use id_arena::Id;
use serde::Serialize;
use std::sync::Arc;

pub(crate) type NodeId = Id<Node>;

#[derive(Debug, Serialize)]
pub struct Node {
    pub(crate) path: NodePath,
    pub(crate) name: ArcSwap<Arc<str>>,
    pub(crate) deps: ArcSwapOption<Arc<[Dependency]>>,
    pub(crate) kind: NodeDataKind,
}

impl Node {
    pub(crate) async fn new(
        root: RootPath,
        path: Box<Utf8Path>,
        text: &str,
    ) -> Result<Option<Self>> {
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

    pub(crate) fn hydrate(&self, index: &Index) -> Result<()> {
        let deps = self.deps.load();
        let Some(deps) = deps.as_deref() else {
            return Ok(());
        };
        for dep in deps.as_ref() {
            dep.hydrate(&self.path, index)?;
        }
        Ok(())
    }

    // FIXME: This is a truly terrible return type.
    pub fn name(&self) -> Arc<Arc<str>> {
        self.name.load().clone()
    }

    pub async fn read(&self) -> Result<NodeContent> {
        let text = util::read_to_string(&self.path.abs).await?;
        let data = self.kind.create(&self.path, &text)?;
        Ok(NodeContent { text, data })
    }
}

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct NodeContent {
    pub text: String,
    pub data: NodeData,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum NodeData {
    Document(Document),
}

impl NodeDataTrait for NodeData {
    fn name(&self) -> Arc<str> {
        match self {
            Self::Document(d) => d.name(),
        }
    }

    fn deps(&self) -> Option<Arc<[Dependency]>> {
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
    fn deps(&self) -> Option<Arc<[Dependency]>>;
}
