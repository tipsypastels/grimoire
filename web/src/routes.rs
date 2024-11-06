use crate::app::App;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod index;
mod node;

pub fn router(app: App) -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/node/:path", get(node::get))
        .fallback_service(ServeDir::new(concat!(env!("OUT_DIR"), "/public")))
        .with_state(app)
}

#[allow(unused)]
mod prelude {
    pub(crate) use crate::render::fa;
    pub use crate::render::{Globals, OrNotFound, ServeError, ServeResult};
    pub use askama_axum::Template;
    pub use axum::response::{IntoResponse, Response};
    pub use grimoire_core::{Grimoire, Node, NodeHead};
}
