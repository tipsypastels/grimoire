use crate::{
    index::Index,
    node::{Node, NodeId},
    path::{NodePath, NodePathRel},
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use hashbrown::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::OnceLock;

#[derive(Debug, Default, Serialize)]
pub struct Dependencies<'a>(pub(crate) HashMap<NodePathRel, &'a Node>);

#[derive(Debug)]
pub(crate) struct Dependency {
    rel: Box<Utf8Path>,
    id: OnceLock<NodeId>,
}

impl Dependency {
    pub fn id(&self) -> NodeId {
        *self.id.get().expect("got unhydrated dependency id")
    }

    pub fn hydrate(&self, from: &NodePath, index: &Index) -> Result<()> {
        let to = from.dependency(&self.rel)?;
        let id = index
            .get(&to)
            .with_context(|| format!("unknown dependency {from}->{to}"))?;

        tracing::debug!(%from, %to, "dependency");
        self.id.set(id).expect("cannot rehydrate dependency");

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
