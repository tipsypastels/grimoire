use anyhow::Result;
use pulldown_cmark::Parser;

pub use pulldown_cmark::Options as MarkdownOptions;

pub trait Markdown {
    fn options(&self) -> MarkdownOptions;
}

impl Markdown for () {
    fn options(&self) -> MarkdownOptions {
        MarkdownOptions::empty()
    }
}

pub fn markdown(text: &str) -> Result<String> {
    markdown_with(text, ())
}

pub fn markdown_with<M: Markdown>(text: &str, helper: M) -> Result<String> {
    let options = helper.options();
    let parser = Parser::new_ext(text, options);
    let mut output = String::new();

    pulldown_cmark::html::push_html(&mut output, parser);
    Ok(output)
}
