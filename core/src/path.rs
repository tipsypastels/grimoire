use crate::memory::AsMemoryMapKey;
use anyhow::{Context, Result};
use camino::Utf8Path;
use std::{borrow::Borrow, fmt, ops::Deref, path::Path, sync::Arc};

macro_rules! as_ref_path_newtype {
    ($ty:ty) => {
        impl AsRef<Path> for $ty {
            fn as_ref(&self) -> &Path {
                self.0.as_std_path()
            }
        }

        impl AsRef<Utf8Path> for $ty {
            fn as_ref(&self) -> &Utf8Path {
                &self.0
            }
        }

        impl Borrow<Path> for $ty {
            fn borrow(&self) -> &Path {
                self.as_ref()
            }
        }

        impl Borrow<Utf8Path> for $ty {
            fn borrow(&self) -> &Utf8Path {
                self.as_ref()
            }
        }

        impl Deref for $ty {
            type Target = Utf8Path;

            fn deref(&self) -> &Utf8Path {
                self.as_ref()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct RootPath(Arc<Utf8Path>);

impl RootPath {
    pub fn new(path: Arc<Utf8Path>) -> Self {
        Self(path)
    }
}

as_ref_path_newtype!(RootPath);

impl fmt::Display for RootPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct EntryPath {
    pub root: RootPath,
    pub abs: EntryPathAbs,
    // Arc because it's the MemoryMap key.
    pub rel: EntryPathRel,
}

impl EntryPath {
    pub fn new(root: RootPath, abs: Box<Utf8Path>) -> Result<Self> {
        let rel = abs
            .strip_prefix(&root)
            .with_context(|| format!("path {abs} is not in root {root}"))?
            .into();

        let abs = EntryPathAbs(abs);
        let rel = EntryPathRel(rel);

        Ok(Self { root, abs, rel })
    }

    pub fn dependency(&self, dep: &Utf8Path) -> Result<Self> {
        let parent = self.abs.parent().expect("path without parent");
        let abs = parent
            .join(dep)
            .canonicalize_utf8()
            .with_context(|| format!("couldn't canonicalize dependency {} from {}", dep, self))?;

        Self::new(self.root.clone(), abs.into())
    }

    pub fn extension(&self) -> Option<&str> {
        self.abs.extension()
    }
}

impl AsMemoryMapKey for EntryPath {
    fn as_memory_map_key(&self) -> &Utf8Path {
        self.rel.as_memory_map_key()
    }
}

impl fmt::Display for EntryPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rel)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntryPathAbs(Box<Utf8Path>);

as_ref_path_newtype!(EntryPathAbs);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryPathRel(Arc<Utf8Path>);

as_ref_path_newtype!(EntryPathRel);

impl AsMemoryMapKey for EntryPathRel {
    fn as_memory_map_key(&self) -> &Utf8Path {
        self
    }
}

impl fmt::Display for EntryPathRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
