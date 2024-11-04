use crate::app::App;
use axum::{routing::get, Router};

mod index;

pub fn router(app: App) -> Router {
    Router::new().route("/", get(index::get)).with_state(app)
}
