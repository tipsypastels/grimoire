use crate::db::DbNodePath;
use anyhow::{Context, Result};
use camino::Utf8Path;
use std::{borrow::Borrow, fmt, ops::Deref, path::Path};

#[derive(Debug)]
pub struct NodePath {
    rel: NodePathRel,
    abs: NodePathAbs,
}

impl NodePath {
    pub(crate) fn new(root: &Utf8Path, abs: Box<Utf8Path>) -> Result<Self> {
        let rel = abs
            .strip_prefix(root)
            .with_context(|| format!("path {abs} is not in root {root}"))?
            .into();

        let rel = NodePathRel(rel);
        let abs = NodePathAbs(abs);

        Ok(Self { rel, abs })
    }

    pub(crate) fn revive(root: &Utf8Path, path: DbNodePath) -> Self {
        let rel = NodePathRel(path.0);
        let abs = NodePathAbs(root.join(&rel).into());
        Self { rel, abs }
    }

    pub(crate) fn dependency(&self, root: &Utf8Path, dep: &str) -> Result<Self> {
        let parent = self.abs.parent().expect("path should have parent");
        let abs = parent
            .join(dep)
            .canonicalize_utf8()
            .with_context(|| format!("can't canonicalize dep {dep} from {self}"))?;

        Self::new(root, abs.into())
    }

    pub fn rel(&self) -> &NodePathRel {
        &self.rel
    }

    pub fn abs(&self) -> &NodePathAbs {
        &self.abs
    }
}

impl fmt::Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rel)
    }
}

#[derive(Debug)]
pub struct NodePathRel(Box<Utf8Path>);

impl fmt::Display for NodePathRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

path_newtype!(NodePathRel);

#[derive(Debug)]
pub struct NodePathAbs(Box<Utf8Path>);

path_newtype!(NodePathAbs);

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

use path_newtype;
