use crate::arena::ArenaPath;
use anyhow::{Context, Result};
use camino::Utf8Path;
use serde::Serialize;
use std::{borrow::Borrow, fmt, ops::Deref, path::Path, sync::Arc};

macro_rules! path_newtype {
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

path_newtype!(RootPath);

impl fmt::Display for RootPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
pub struct NodePath {
    #[serde(skip_serializing)]
    pub root: RootPath,
    pub abs: NodePathAbs,
    // Arc because it's the MemoryMap key.
    pub rel: NodePathRel,
}

impl NodePath {
    pub fn new(root: RootPath, abs: Box<Utf8Path>) -> Result<Self> {
        let rel = abs
            .strip_prefix(&root)
            .with_context(|| format!("path {abs} is not in root {root}"))?
            .into();

        let abs = NodePathAbs(abs);
        let rel = NodePathRel(rel);

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

impl ArenaPath for NodePath {
    fn arena_path(&self) -> &Utf8Path {
        self.rel.arena_path()
    }
}

impl fmt::Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rel)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct NodePathAbs(Box<Utf8Path>);

path_newtype!(NodePathAbs);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct NodePathRel(Arc<Utf8Path>);

path_newtype!(NodePathRel);

impl ArenaPath for NodePathRel {
    fn arena_path(&self) -> &Utf8Path {
        self
    }
}

impl fmt::Display for NodePathRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
