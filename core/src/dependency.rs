use crate::{
    arena::{ArenaPaths, Id},
    path::{NodePath, NodePathRel},
};
use anyhow::{Context, Result};
use camino::Utf8Path;
use hashbrown::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{ops::Deref, sync::OnceLock};

#[derive(Debug)]
pub struct Dependencies<N>(HashMap<NodePathRel, N>);

impl<N> Default for Dependencies<N> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<N> FromIterator<(NodePathRel, N)> for Dependencies<N> {
    fn from_iter<T: IntoIterator<Item = (NodePathRel, N)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<N> Extend<(NodePathRel, N)> for Dependencies<N> {
    fn extend<T: IntoIterator<Item = (NodePathRel, N)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

// Can't serialize RwLockReadGuard directly.
impl<N> Serialize for Dependencies<N>
where
    N: Deref<Target: Serialize>,
{
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.collect_map(self.0.iter().map(|(k, v)| (k, v.deref())))
    }
}

#[derive(Debug)]
pub(crate) struct Dependency {
    rel: Box<Utf8Path>,
    id: OnceLock<Id>,
}

impl Dependency {
    pub fn id(&self) -> Id {
        *self.id.get().expect("got unhydrated dependency id")
    }

    pub fn hydrate(&self, from: &NodePath, path_map: &ArenaPaths) -> Result<()> {
        let to = from.dependency(&self.rel)?;
        let id = path_map
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
