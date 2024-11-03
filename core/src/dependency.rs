use crate::{
    arena::{ArenaPaths, Id},
    path::NodePath,
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use serde::{Deserialize, Deserializer};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct DependencyRef {
    rel: Box<Utf8Path>,
    id: OnceLock<Id>,
}

impl DependencyRef {
    pub fn hydrate(&self, from: &NodePath, path_map: &ArenaPaths) -> Result<()> {
        let to = from.dependency(&self.rel)?;
        let id = path_map
            .get(&to)
            .with_context(|| format!("unknown dependency {from}->{to}"))?;

        tracing::debug!(%from, %to, "dependency");
        self.id.set(id).expect("cannot rehydrate node");

        Ok(())
    }
}

impl<'de> Deserialize<'de> for DependencyRef {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self {
            rel: Box::<Utf8Path>::deserialize(de)?,
            id: OnceLock::new(),
        })
    }
}
