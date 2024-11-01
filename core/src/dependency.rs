use crate::entry::Entry;
use camino::Utf8Path;
use id_arena::Id;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Dependency {
    rel_path: Box<Utf8Path>,
    id: Option<Id<Entry>>,
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self {
            rel_path: Box::<Utf8Path>::deserialize(de)?,
            id: None,
        })
    }
}
