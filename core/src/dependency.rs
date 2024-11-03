use crate::{
    arena::{ArenaPaths, Id},
    path::NodePath,
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Dependency {
    rel: Box<Utf8Path>,
    id: OnceLock<Id>,
}

impl Dependency {
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

impl Serialize for Dependency {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.rel.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self {
            rel: Box::<Utf8Path>::deserialize(de)?,
            id: OnceLock::new(),
        })
    }
}
