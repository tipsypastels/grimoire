use anyhow::{Context, Result};
use gray_matter::{engine, Matter};

mod body;
mod head;

pub use body::*;
pub use head::*;

#[derive(Debug)]
#[non_exhaustive]
pub struct Document {
    pub head: DocumentHead,
    pub body: DocumentBody,
}

impl Document {
    pub fn new(content: &str, body_kind: DocumentBodyKind) -> Result<Self> {
        let matter = Matter::<engine::YAML>::new();
        let matter = matter
            .parse_with_struct::<DocumentHead>(content)
            .context("no front matter")?;

        let head = matter.data;
        let body = body_kind.into_body(&matter.content)?;

        Ok(Self { head, body })
    }
}
