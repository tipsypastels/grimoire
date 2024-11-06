use crate::node::NodePath;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug)]
pub struct Document {
    head: Head,
    body: Box<str>,
    text: Box<str>,
}

#[derive(Debug, Deserialize)]
struct Head {
    name: Box<str>,
    #[serde(default)]
    deps: Option<Box<[Box<str>]>>,
}

impl Document {
    pub(crate) fn new(path: &NodePath, text: &str) -> Result<Self> {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let matter = matter
            .parse_with_struct::<Head>(text)
            .context("no frontmatter")?;

        let head = matter.data;
        let body = matter.content.into();
        let text = matter.orig.into();

        tracing::debug!(name = %head.name, %path, "document");
        Ok(Self { head, body, text })
    }

    pub fn name(&self) -> &str {
        &self.head.name
    }

    pub(crate) fn deps(&self) -> Option<&[Box<str>]> {
        self.head.deps.as_deref()
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
