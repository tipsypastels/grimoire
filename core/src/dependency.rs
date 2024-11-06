use crate::{
    db::Db,
    node::{NodeData, NodePath},
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use rustc_hash::FxHashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Collector<'a> {
    root: &'a Utf8Path,
    db: &'a Db,
    map: FxHashMap<Key, (i64, NodeData)>,
}

impl<'a> Collector<'a> {
    pub fn new(root: &'a Utf8Path, db: &'a Db) -> Self {
        Self {
            root,
            db,
            map: FxHashMap::default(),
        }
    }

    pub fn collect(&mut self, id: i64, path: NodePath, data: NodeData) {
        self.map.insert(Key(path), (id, data));
    }

    pub async fn populate(self) -> Result<()> {
        for (Key(path), (id, data)) in &self.map {
            let Some(deps) = data.deps() else {
                continue;
            };
            for dep in deps {
                let dep_path = path.dependency(self.root, dep)?;
                let key = Key(dep_path);
                let (dep_id, _) = self
                    .map
                    .get(&key)
                    .with_context(|| format!("unknown dep {} from {path}", key.0))?;

                tracing::debug!(from = %path, to = %key.0, "dependency");
                self.db.insert_node_dependency(*id, *dep_id).await?;
            }
        }

        Ok(())
    }
}

// Keep the entire `NodePath` value but use `NodePathAbs` as the effective key
// so we can canonicalize the dependency paths and use those for lookup.
#[derive(Debug)]
struct Key(NodePath);

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0.abs().as_str() == other.0.abs().as_str()
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.abs().as_str().hash(state);
    }
}
