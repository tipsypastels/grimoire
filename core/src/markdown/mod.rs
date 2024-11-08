use anyhow::{Context, Result};
use comrak::{Arena, Options};

mod heading;
mod toc;
mod util;

pub use toc::{Toc, TocNode};

#[derive(Debug)]
pub struct Markdown {
    html: String,
}

impl Markdown {
    pub(crate) fn new<H: Hooks>(text: &str) -> Result<Self> {
        let arena = Arena::new();
        let mut options = Options::default();
        // Allow rendering raw HTML.
        options.render.unsafe_ = true;

        let root = comrak::parse_document(&arena, text, &options);
        let mut toc = Toc::new();

        heading::process::<H>(root, &mut toc);

        let mut out = Vec::new();
        comrak::html::format_document(root, &options, &mut out)
            .context("markdown formatting error")?;

        let html = String::from_utf8(out).context("markdown not UTF-8")?;
        Ok(Self { html })
    }

    pub fn html(&self) -> &str {
        &self.html
    }
}

pub trait Hooks {
    fn render_toc(toc: &Toc) -> String {
        format!("<pre>{toc:?}</pre>")
    }
}

impl Hooks for () {}
