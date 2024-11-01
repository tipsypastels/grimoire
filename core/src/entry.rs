use crate::document::Document;
use anyhow::{Context, Result};
use camino::Utf8Path;
use enum_dispatch::enum_dispatch;
use std::sync::Arc;

#[derive(Debug)]
pub struct Entry {
    pub path: Arc<Utf8Path>,
    pub text: Option<Arc<str>>,
    pub data: Option<EntryData>,
    pub ignored: bool,
    pub deleted: bool,
}

impl Entry {
    pub fn new(path: Arc<Utf8Path>, text: Option<Arc<str>>) -> Result<Self> {
        let (data, ignored) = if let Some(text) = text.as_ref() {
            match Self::new_data(&path, text)? {
                Some(data) => (Some(data), false),
                None => (None, true),
            }
        } else {
            (None, false)
        };

        Ok(Self {
            path,
            text,
            data,
            ignored,
            deleted: false,
        })
    }

    fn new_data(path: &Utf8Path, text: &str) -> Result<Option<EntryData>> {
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
}

#[derive(Debug)]
#[enum_dispatch(EntryType)]
pub enum EntryData {
    Document,
}

#[enum_dispatch]
pub trait EntryType: Into<EntryData> {}
