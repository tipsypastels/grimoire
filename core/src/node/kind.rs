use super::NodePath;
use crate::document::Document;
use anyhow::{anyhow, Context, Result};
use std::{fmt, str::FromStr};

kinds! {
    Some("md") => Document,
}

macro_rules! kinds {
    ($($pat:pat => $kind:ident),*$(,)?) => {
        #[derive(Debug)]
        pub enum NodeData {
            $($kind($kind)),*
        }

        impl NodeData {
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$kind(x) => x.name(),)*
                }
            }

             pub fn deps(&self) -> Option<&[Box<str>]> {
                match self {
                    $(Self::$kind(x) => x.deps(),)*
                }
            }

            pub fn text(&self) -> &str {
                match self {
                    $(Self::$kind(x) => x.text(),)*
                }
            }

            pub fn kind(&self) -> NodeKind {
                match self {
                    $(Self::$kind(_) => NodeKind::$kind,)*
                }
            }
        }

        $(
            impl From<$kind> for NodeData {
                fn from(value: $kind) -> Self {
                    Self::$kind(value)
                }
            }
        )*

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum NodeKind {
            $($kind,)*
        }

        impl NodeKind {
            pub(crate) fn create(self, path: &NodePath, text: &str) -> Result<NodeData> {
                match self {
                    $(
                        Self::$kind => Ok(NodeData::$kind($kind::new(path, text).with_context(|| format!(concat!("failed to create ", stringify!($kind), " {}"), path))?)),
                    )*
                }
            }

            pub(crate) fn determine(extension: Option<&str>) -> Option<Self> {
                match extension {
                    $($pat => Some(Self::$kind),)*
                    _ => None,
                }
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$kind => stringify!($kind),)*
                }
            }
        }

        impl FromStr for NodeKind {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $(stringify!($kind) => Ok(Self::$kind),)*
                    _ => Err(anyhow!("no such NodeKind: `{s}`")),
                }
            }
        }

        impl fmt::Display for NodeKind {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

use kinds;
