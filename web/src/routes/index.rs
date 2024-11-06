use super::prelude::*;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub struct IndexTemplate {
    globals: Globals,
}

pub async fn get(globals: Globals) -> ServeResult<IndexTemplate> {
    Ok(IndexTemplate { globals })
}
