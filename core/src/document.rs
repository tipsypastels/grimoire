use crate::{dependency::Dependency, node::NodeType, path::NodePath};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug)]
#[non_exhaustive]
pub struct Document {
    pub head: DocumentHead,
    pub body: DocumentBody,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct DocumentHead {
    pub name: Box<str>,
    #[serde(default)]
    pub(crate) deps: Option<Box<[Dependency]>>,
}

#[derive(Debug)]
pub enum DocumentBody {
    Md(String),
}

impl Document {
    pub(crate) fn new(path: &NodePath, text: &str) -> Result<Self> {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let matter = matter
            .parse_with_struct::<DocumentHead>(text)
            .context("no front matter")?;

        let head = matter.data;
        let body = DocumentBody::Md(matter.content);

        tracing::debug!(name = head.name, %path, "document");
        Ok(Self { head, body })
    }
}

impl NodeType for Document {
    fn name(&self) -> Option<&str> {
        Some(&self.head.name)
    }

    fn dependencies(&self) -> Option<&[Dependency]> {
        self.head.deps.as_deref()
    }

    fn dependencies_mut(&mut self) -> Option<&mut [Dependency]> {
        self.head.deps.as_deref_mut()
    }
}
