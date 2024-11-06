use crate::render::fa;
use askama_axum::Template;
use axum::response::{IntoResponse, Response};
use grimoire_core::{Grimoire, NodeHead};

pub async fn get(grimoire: Grimoire) -> Response {
    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    pub struct IndexHtml {
        nodes: Vec<NodeHead>,
    }

    let nodes = grimoire.iter().await.unwrap();
    let template = IndexHtml { nodes };

    template.into_response()
}
