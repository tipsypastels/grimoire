use crate::{memory::MemoryMap, node::Node, path::NodePath};
use anyhow::{Context, Result};
use camino::Utf8Path;
use id_arena::Id;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Dependency {
    rel: Box<Utf8Path>,
    id: Option<Id<Node>>,
}

impl Dependency {
    pub fn hydrate(&mut self, from: &NodePath, map: &MemoryMap) -> Result<()> {
        let to = from.dependency(&self.rel)?;
        let id = map
            .get(&to)
            .with_context(|| format!("unknown dependency {to} for {from}"))?;

        tracing::debug!(%from, %to, "dependency");
        self.id = Some(id);
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self {
            rel: Box::<Utf8Path>::deserialize(de)?,
            id: None,
        })
    }
}
