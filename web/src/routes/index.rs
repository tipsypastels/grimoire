use super::prelude::*;

pub async fn get(grimoire: Grimoire) -> ServeResult<Response> {
    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    pub struct IndexHtml {
        nodes: Vec<NodeHead>,
    }

    let nodes = grimoire.all().await?;
    let template = IndexHtml { nodes };

    Ok(template.into_response())
}
