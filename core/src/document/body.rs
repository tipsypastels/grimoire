use anyhow::Result;

#[derive(Debug)]
pub enum DocumentBody {
    Md(Box<str>),
    Mdx(Box<str>),
}

#[derive(Debug, Copy, Clone)]
pub enum DocumentBodyKind {
    Md,
    Mdx,
}

impl DocumentBodyKind {
    pub(super) fn into_body(self, content: &str) -> Result<DocumentBody> {
        match self {
            Self::Md => Ok(DocumentBody::Md(content.into())),
            Self::Mdx => Ok(DocumentBody::Mdx(content.into())),
        }
    }
}
