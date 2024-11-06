use crate::app::App;
use axum::{extract::Request, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::Span;

mod index;
mod node;

pub fn router(app: App) -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/node/:path", get(node::get))
        .fallback_service(ServeDir::new(concat!(env!("OUT_DIR"), "/public")))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(make_span)))
        .with_state(app)
}

pub fn make_span(request: &Request) -> Span {
    let span = tracing::info_span!(
        "request",
        method = %request.method(),
        path = %request.uri().path(),
        turbo = tracing::field::Empty,
    );
    if crate::serve::is_turbo_frame(request.headers()) {
        span.record("turbo", true);
    }
    span
}

#[allow(unused)]
mod prelude {
    pub(crate) use crate::render::fa;
    pub use crate::serve::{Globals, OrNotFound, ServeError, ServeResult};
    pub use askama_axum::Template;
    pub use axum::response::{IntoResponse, Response};
    pub use grimoire_core::{Grimoire, Node, NodeHead};
}
