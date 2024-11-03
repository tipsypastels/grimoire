use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use camino::Utf8PathBuf;
use grimoire::Grimoire;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
struct App {
    grimoire: Grimoire,
}

pub async fn serve(grimoire: Grimoire, port: u16) -> Result<()> {
    let app = App { grimoire };
    let router = Router::new()
        .route("/", get(index))
        .route("/:path", get(path))
        .with_state(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "serving");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index() -> &'static str {
    "Hello, world!"
}

async fn path(State(app): State<App>, Path(path): Path<Utf8PathBuf>) -> Response {
    let Some(node) = app.grimoire.get(&path).await else {
        return StatusCode::NOT_FOUND.into_response();
    };
    dbg!(node);

    "Hello, world".into_response()
}
