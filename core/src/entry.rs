use crate::{dependency::Dependency, document::Document, MemoryMap};
use anyhow::{Context, Result};
use camino::Utf8Path;
use enum_dispatch::enum_dispatch;

#[derive(Debug)]
pub struct Entry {
    pub path: Box<Utf8Path>,
    pub text: Option<Box<str>>,
    pub data: Option<EntryData>,
    pub ignored: bool,
    pub deleted: bool,
}

impl Entry {
    pub fn new(path: Box<Utf8Path>, text: Option<Box<str>>) -> Result<Self> {
        let (data, ignored) = if let Some(text) = text.as_ref() {
            match Self::new_data(&path, text)? {
                Some(data) => (Some(data), false),
                None => (None, true),
            }
        } else {
            (None, false)
        };

        Ok(Self {
            path,
            text,
            data,
            ignored,
            deleted: false,
        })
    }

    fn new_data(path: &Utf8Path, text: &str) -> Result<Option<EntryData>> {
        macro_rules! match_data {
            ($($pat:pat => $name:literal @ <$ty:ty>),*$(,)?) => {
                match path.extension() {
                    $(
                        Some($pat) => {
                            let data = <$ty>::new(path, text)
                                .with_context(|| format!(concat!("failed to create ", $name, " {}"), path))?;

                            Ok(Some(data.into()))
                        },
                    )*
                    Some(_) => Ok(None),
                    None => Ok(None),
                }
            };
        }

        match_data! {
            "md" | "mdx" => "document" @ <Document>,
        }
    }

    pub fn hydrate(&mut self, map: &MemoryMap) -> Result<()> {
        let Some(data) = &mut self.data else {
            return Ok(());
        };
        let Some(deps) = data.dependencies_mut() else {
            return Ok(());
        };
        for dep in deps {
            dep.hydrate(&self.path, map)?;
        }
        Ok(())
    }

    pub fn rel_path(&self, root: &Utf8Path) -> Result<&Utf8Path> {
        let path = &self.path;
        path.strip_prefix(root)
            .with_context(|| format!("path {path} is not in root dir {root}"))
    }
}

#[derive(Debug)]
#[enum_dispatch(EntryType)]
pub enum EntryData {
    Document,
}

#[enum_dispatch]
pub trait EntryType: Into<EntryData> {
    fn dependencies(&self) -> Option<&[Dependency]> {
        None
    }

    fn dependencies_mut(&mut self) -> Option<&mut [Dependency]> {
        None
    }
}
