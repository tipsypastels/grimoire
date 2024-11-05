use self::app::App;
use anyhow::Result;
use grimoire_core::Grimoire;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod app;
mod grimoire;
mod routes;

pub async fn serve(grimoire: Grimoire, port: u16) -> Result<()> {
    let app = App::new(grimoire);
    let router = routes::router(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "serving");
    axum::serve(listener, router).await?;

    Ok(())
}
