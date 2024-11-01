use crate::entry::EntryType;
use anyhow::{Context, Result};
use camino::Utf8Path;
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
}

#[derive(Debug)]
pub enum DocumentBody {
    Md(String),
}

impl Document {
    pub(crate) fn new(_path: &Utf8Path, text: &str) -> Result<Self> {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let matter = matter
            .parse_with_struct::<DocumentHead>(text)
            .context("no front matter")?;

        let head = matter.data;
        let body = DocumentBody::Md(matter.content);

        Ok(Self { head, body })
    }
}

impl EntryType for Document {}
