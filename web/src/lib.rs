use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use camino::Utf8PathBuf;
use grimoire::{Dependencies, Grimoire, Node};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

#[derive(Debug, Clone)]
struct App {
    grimoire: Arc<RwLock<Grimoire>>,
}

pub async fn serve(grimoire: Grimoire, port: u16) -> Result<()> {
    let app = App {
        grimoire: Arc::new(RwLock::new(grimoire)),
    };
    let router = Router::new()
        .route("/", get(index))
        .route("/:page", get(page))
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

async fn page(State(app): State<App>, Path(page): Path<Utf8PathBuf>) -> Response {
    #[derive(Serialize)]
    struct PageJson<'a> {
        node: &'a Node,
        deps: Option<&'a Dependencies<'a>>,
    }

    let grimoire = app.grimoire.read().await;
    let Some(node) = grimoire.get(page) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let deps = grimoire.deps(node);
    let json = PageJson {
        node,
        deps: deps.as_ref(),
    };

    Json(json).into_response()
}
