use super::prelude::*;
use axum::extract::Path;
use camino::Utf8PathBuf;

#[derive(Template)]
#[template(path = "node.html")]
pub struct NodeTemplate {
    globals: Globals,
    node: Node,
    markdown: String,
}

pub async fn get(
    grimoire: Grimoire,
    globals: Globals,
    Path(path): Path<Utf8PathBuf>,
) -> ServeResult<NodeTemplate> {
    let node = grimoire.get(&path).await?;
    let node = node.or_not_found()?;
    let markdown = grimoire_core::markdown(match node.data() {
        grimoire_core::NodeData::Document(document) => document.body(),
    })?;

    Ok(NodeTemplate {
        globals,
        node,
        markdown,
    })
}
