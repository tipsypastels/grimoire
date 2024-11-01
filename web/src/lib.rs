use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};
use grimoire::Grimoire;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct App {
    grimoire: Grimoire,
}

pub async fn serve(grimoire: Grimoire, port: u16) -> Result<()> {
    let app = App { grimoire };
    let router = Router::new().route("/", get(index)).with_state(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "serving");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index() -> &'static str {
    "Hello, world!"
}
