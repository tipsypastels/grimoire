use crate::{
    grimoire::GrimoireLock,
    render::{fa, CellIter},
};
use askama_axum::Template;
use axum::response::{IntoResponse, Response};
use grimoire_core::Nodes;

pub async fn get(grimoire: GrimoireLock) -> Response {
    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    pub struct IndexHtml<'a> {
        nodes: CellIter<Nodes<'a>>,
    }

    let grimoire = grimoire.read().await;
    let nodes = grimoire.nodes().into();
    let template = IndexHtml { nodes };

    template.into_response()
}
