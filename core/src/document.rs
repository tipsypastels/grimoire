use crate::{dependency::DependencyRef, path::NodePath, NodeDataTrait};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct Document {
    pub head: DocumentHead,
    pub body: DocumentBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentHead {
    pub name: Arc<str>,
    #[serde(default, skip_serializing)]
    pub(crate) deps: Option<Arc<[DependencyRef]>>,
}

#[derive(Debug, Serialize)]
pub enum DocumentBody {
    Md(Box<str>),
}

impl Document {
    pub fn new(path: &NodePath, text: &str) -> Result<Self> {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let matter = matter
            .parse_with_struct::<DocumentHead>(text)
            .context("no front matter")?;

        let head = matter.data;
        let body = DocumentBody::Md(matter.content.into());

        tracing::debug!(name = %head.name, %path, "document");
        Ok(Self { head, body })
    }
}

impl NodeDataTrait for Document {
    fn name(&self) -> Arc<str> {
        self.head.name.clone()
    }

    fn dependency_refs(&self) -> Option<Arc<[DependencyRef]>> {
        self.head.deps.clone()
    }
}
