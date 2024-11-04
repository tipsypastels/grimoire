use crate::app::App;
use axum::{
    http::{header, HeaderName},
    routing::get,
    Router,
};

pub fn router() -> Router<App> {
    Router::new().route("/tailwind.css", get(tailwind_css))
}

async fn tailwind_css() -> ([(HeaderName, &'static str); 1], &'static str) {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!(concat!(env!("OUT_DIR"), "/tailwind.css")),
    )
}
