use crate::document::{Document, DocumentBodyKind};
use anyhow::{Context, Error, Result};
use camino::Utf8Path;
use std::sync::Arc;
use tokio::fs;

#[derive(Debug)]
pub struct Entry {
    pub path: Arc<Utf8Path>,
    pub deleted: bool,
    state: State,
}

impl Entry {
    pub fn new(path: Arc<Utf8Path>) -> Self {
        Self {
            path,
            deleted: false,
            state: State::Unread,
        }
    }

    pub async fn new_with_content(path: Arc<Utf8Path>) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .await
            .with_context(|| format!("failed to read entry {path}"))?;
        let mut entry = Self::new(path);
        entry.set_content(&content)?;
        Ok(entry)
    }

    pub fn set_content(&mut self, content: &str) -> Result<()> {
        self.state = match self.path.extension() {
            Some("md") => State::Document(
                Document::new(content, DocumentBodyKind::Md)
                    .with_context(|| format!("failed to read document {}", self.path))?,
            ),
            Some("mdx") => State::Document(
                Document::new(content, DocumentBodyKind::Mdx)
                    .with_context(|| format!("failed to read document {}", self.path))?,
            ),

            _ => State::Unknown,
        };
        Ok(())
    }

    pub fn set_error(&mut self, error: Error) {
        self.state = State::Error(error);
    }
}

#[derive(Debug)]
enum State {
    Document(Document),
    Error(Error),
    Unknown,
    Unread,
}
