use super::{ServeError, ServeResult};
use crate::app::App;
use axum::{extract::FromRequestParts, http::request::Parts};
use grimoire_core::NodeHead;

#[derive(Debug)]
#[allow(clippy::manual_non_exhaustive)]
pub struct Globals {
    pub nodes: Vec<NodeHead>,
    pub is_turbo_frame: bool,
    _priv: (),
}

#[axum::async_trait]
impl FromRequestParts<App> for Globals {
    type Rejection = ServeError;

    async fn from_request_parts(parts: &mut Parts, app: &App) -> ServeResult<Self> {
        let nodes = app.grimoire.all().await?;
        let is_turbo_frame = super::is_turbo_frame(&parts.headers);

        Ok(Self {
            nodes,
            is_turbo_frame,
            _priv: (),
        })
    }
}
