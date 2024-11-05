use crate::app::App;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod index;

pub fn router(app: App) -> Router {
    Router::new()
        .route("/", get(index::get))
        .fallback_service(ServeDir::new(concat!(env!("OUT_DIR"), "/public")))
        .with_state(app)
}
