use crate::{entry::Entry, memory::MemoryMap};
use anyhow::{Context, Result};
use camino::Utf8Path;
use id_arena::Id;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Dependency {
    rel_path: Box<Utf8Path>,
    id: Option<Id<Entry>>,
}

impl Dependency {
    pub fn hydrate(&mut self, entry_path: &Utf8Path, map: &MemoryMap) -> Result<()> {
        let rel_path = &self.rel_path;
        let path = entry_path
            .parent()
            .context("path without parent")?
            .join(&self.rel_path)
            .canonicalize_utf8()
            .with_context(|| {
                format!("couldn't canonicalize {rel_path} for {entry_path} while hydrating")
            })?;

        let id = map
            .id_abs(path)
            .with_context(|| format!("unknown dependency {rel_path} for {entry_path}"))?;

        self.id = Some(id);
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self {
            rel_path: Box::<Utf8Path>::deserialize(de)?,
            id: None,
        })
    }
}
