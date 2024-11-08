use super::prelude::*;
use axum::extract::Path;
use camino::Utf8PathBuf;
use grimoire_core::markdown::Markdown;

#[derive(Template)]
#[template(path = "node.html")]
pub struct NodeTemplate {
    globals: Globals,
    node: Node,
    markdown: Markdown,
}

pub async fn get(
    grimoire: Grimoire,
    globals: Globals,
    Path(path): Path<Utf8PathBuf>,
) -> ServeResult<NodeTemplate> {
    let node = grimoire.get(&path).await?;
    let node = node.or_not_found()?;
    let markdown = match node.data() {
        grimoire_core::node::NodeData::Document(document) => document.markdown::<()>(),
    }?;

    Ok(NodeTemplate {
        globals,
        node,
        markdown,
    })
}
