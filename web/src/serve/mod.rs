mod error;
mod globals;

pub use error::{OrNotFound, ServeError, ServeResult};
pub use globals::Globals;

pub fn is_turbo_frame(headers: &axum::http::HeaderMap) -> bool {
    headers.contains_key("Turbo-Frame")
}