use crate::app::App;
use axum::{routing::get, Router};

mod artifact;
mod index;

pub fn router(app: App) -> Router {
    Router::new()
        .route("/", get(index::get))
        .merge(artifact::router())
        .with_state(app)
}
