use crate::app::App;
use axum::{
    http::{header, HeaderName},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

pub fn router() -> Router<App> {
    let js = ServeDir::new(concat!(env!("OUT_DIR"), "/js"));
    Router::new()
        .route("/tailwind.css", get(tailwind_css))
        .fallback_service(js)
}

async fn tailwind_css() -> ([(HeaderName, &'static str); 1], &'static str) {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!(concat!(env!("OUT_DIR"), "/tailwind.css")),
    )
}
