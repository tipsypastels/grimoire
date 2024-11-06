use crate::render::{fa, ServeResult};
use askama_axum::Template;
use axum::response::{IntoResponse, Response};
use grimoire_core::{Grimoire, NodeHead};

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
