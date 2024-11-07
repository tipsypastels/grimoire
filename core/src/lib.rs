use self::db::Db;
use anyhow::{Error, Result};
use camino::Utf8Path;
use futures::{StreamExt, TryStreamExt};
use std::sync::Arc;

mod db;
mod dependency;
mod document;
mod markdown;
mod node;
mod util;

pub use document::*;
pub use markdown::*;
pub use node::*;

#[derive(Debug, Clone)]
pub struct Grimoire {
    root: Arc<Utf8Path>,
    db: Db,
}

impl Grimoire {
    pub async fn new(root: impl Into<Arc<Utf8Path>>) -> Result<Self> {
        let root = root.into();
        let db = Db::new().await?;

        Ok(Self { root, db })
    }

    pub async fn get(&self, path: impl AsRef<Utf8Path>) -> Result<Option<Node>> {
        let path = path.as_ref().as_str();
        let Some(node) = self.db.get_node_by_path(path).await? else {
            return Ok(None);
        };
        Ok(Some(Node::revive(&self.root, node)?))
    }

    pub async fn all(&self) -> Result<Vec<NodeHead>> {
        self.db
            .get_nodes()
            .map_err(Error::from)
            .and_then(|node| async move { NodeHead::revive(&self.root, node) })
            .try_collect()
            .await
    }

    pub async fn insert(
        &self,
        path: impl Into<Box<Utf8Path>>,
        text: &str,
    ) -> Result<Option<(i64, NodePath, NodeData)>> {
        let path = NodePath::new(&self.root, path.into())?;
        let Some(kind) = NodeKind::determine(path.abs().extension()) else {
            return Ok(None);
        };
        let data = kind.create(&path, text)?;
        let node = NewNode {
            path: &path,
            data: &data,
        };

        tracing::info!(name = %data.name(), %path, "node");
        let id = self.db.insert_node(node.into()).await?;
        Ok(Some((id, path, data)))
    }

    pub async fn populate(&self) -> Result<()> {
        let mut stream = util::walk_dir_and_read(&self.root).await;
        let mut collector = dependency::Collector::new(&self.root, &self.db);
        while let Some(result) = stream.next().await {
            let (path, text) = result?;
            let Some((id, path, data)) = self.insert(path, &text).await? else {
                continue;
            };
            collector.collect(id, path, data);
        }
        collector.populate().await?;
        Ok(())
    }
}
