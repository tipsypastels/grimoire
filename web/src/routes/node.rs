use super::prelude::*;
use axum::extract::Path;
use camino::Utf8PathBuf;

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    node: Node,
}

pub async fn get(grimoire: Grimoire, Path(path): Path<Utf8PathBuf>) -> ServeResult<PageTemplate> {
    let node = grimoire.get(&path).await?;
    let node = node.or_not_found()?;

    Ok(PageTemplate { node })
}
