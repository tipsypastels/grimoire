use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct DocumentHead {
    pub title: Box<str>,
}
