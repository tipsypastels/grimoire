use crate::{entry::Entry, memory::MemoryMap, path::EntryPath};
use anyhow::{Context, Result};
use camino::Utf8Path;
use id_arena::Id;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Dependency {
    rel: Box<Utf8Path>,
    id: Option<Id<Entry>>,
}

impl Dependency {
    pub fn hydrate(&mut self, entry_path: &EntryPath, map: &MemoryMap) -> Result<()> {
        let rel = &self.rel;
        let path = entry_path.dependency(rel)?;
        let id = map
            .get(&path)
            .with_context(|| format!("unknown dependency {path} for {entry_path}"))?;

        tracing::debug!(from = %entry_path, to = %path, "dependency");
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
