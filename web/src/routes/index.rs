use crate::{grimoire::GrimoireLock, render::fa};
use askama_axum::Template;
use axum::response::{IntoResponse, Response};
use grimoire_core::Node;

pub async fn get(grimoire: GrimoireLock) -> Response {
    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    pub struct IndexHtml<'a> {
        nodes: Vec<&'a Node>,
    }

    let grimoire = grimoire.read().await;
    let nodes = grimoire.nodes().collect();
    let template = IndexHtml { nodes };

    template.into_response()
}
