use crate::{
    dependency::Dependency,
    document::Document,
    memory::MemoryMap,
    path::{NodePath, RootPath},
};
use anyhow::{Context, Result};
use camino::Utf8Path;

#[derive(Debug)]
pub struct Node {
    pub path: NodePath,
    pub text: Option<Box<str>>,
    pub data: Option<NodeData>,
    pub ignored: bool,
    pub deleted: bool,
}

impl Node {
    pub fn new(root: RootPath, path: Box<Utf8Path>, text: Option<Box<str>>) -> Result<Self> {
        let path = NodePath::new(root, path)?;
        let (data, ignored) = if let Some(text) = text.as_ref() {
            match Self::new_data(&path, text)? {
                Some(data) => (Some(data), false),
                None => (None, true),
            }
        } else {
            (None, false)
        };

        if !ignored {
            let name = data.as_ref().and_then(|d| d.name());
            tracing::debug!(name, %path, "node");
        }

        Ok(Self {
            path,
            text,
            data,
            ignored,
            deleted: false,
        })
    }

    fn new_data(path: &NodePath, text: &str) -> Result<Option<NodeData>> {
        macro_rules! match_data {
            ($($pat:pat => $name:literal @ <$ty:ty>),*$(,)?) => {
                match path.extension() {
                    $(
                        Some($pat) => {
                            let data = <$ty>::new(path, text)
                                .with_context(|| format!(concat!("failed to create ", $name, " {}"), path))?;

                            Ok(Some(data.into()))
                        },
                    )*
                    Some(_) => Ok(None),
                    None => Ok(None),
                }
            };
        }

        match_data! {
            "md" | "mdx" => "document" @ <Document>,
        }
    }

    pub fn hydrate(&mut self, map: &MemoryMap) -> Result<()> {
        let Some(data) = &mut self.data else {
            return Ok(());
        };
        let Some(deps) = data.dependencies_mut() else {
            return Ok(());
        };
        for dep in deps {
            dep.hydrate(&self.path, map)?;
        }
        Ok(())
    }
}

pub trait NodeType: Into<NodeData> {
    fn name(&self) -> Option<&str> {
        None
    }

    fn dependencies(&self) -> Option<&[Dependency]> {
        None
    }

    fn dependencies_mut(&mut self) -> Option<&mut [Dependency]> {
        None
    }
}

#[derive(Debug)]
pub enum NodeData {
    Document(Document),
}

impl NodeType for NodeData {
    fn name(&self) -> Option<&str> {
        match self {
            Self::Document(document) => document.name(),
        }
    }

    fn dependencies(&self) -> Option<&[Dependency]> {
        match self {
            Self::Document(document) => document.dependencies(),
        }
    }

    fn dependencies_mut(&mut self) -> Option<&mut [Dependency]> {
        match self {
            Self::Document(document) => document.dependencies_mut(),
        }
    }
}

impl From<Document> for NodeData {
    fn from(document: Document) -> Self {
        Self::Document(document)
    }
}
