use anyhow::{Context, Result};
use camino::Utf8Path;
use std::{fmt, path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub struct RootPath(Arc<Utf8Path>);

impl RootPath {
    pub fn new(path: Arc<Utf8Path>) -> Self {
        Self(path)
    }
}

impl AsRef<Path> for RootPath {
    fn as_ref(&self) -> &Path {
        self.0.as_std_path()
    }
}

impl AsRef<Utf8Path> for RootPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl fmt::Display for RootPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct EntryPath {
    pub root: RootPath,
    pub abs: Box<Utf8Path>,
    // Arc because it's the MemoryMap key.
    pub rel: Arc<Utf8Path>,
}

impl EntryPath {
    pub fn new(root: RootPath, abs: Box<Utf8Path>) -> Result<Self> {
        let rel = abs
            .strip_prefix(&root)
            .with_context(|| format!("path {abs} is not in root {root}"))?
            .into();

        Ok(Self { root, abs, rel })
    }
}

impl fmt::Display for EntryPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rel)
    }
}
