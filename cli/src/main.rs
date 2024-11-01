use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::Parser;
use dotenvy::dotenv;
use grimoire::Grimoire;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Parser)]
struct Opts {
    dir: Option<Utf8PathBuf>,

    #[clap(long = "port", env = "PORT", default_value = "5173")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let opts = Opts::parse();

    setup_tracing()?;
    tracing::debug!("{opts:#?}");

    let dir = opts
        .dir
        .or_else(|| env::current_dir().ok()?.try_into().ok())
        .and_then(|d| d.canonicalize_utf8().ok())
        .context("invalid directory")?;

    tracing::debug!(%dir, "serving");

    let grimoire = Grimoire::builder(dir).walk_and_read().await?.build();
    dbg!(grimoire);

    tracing::info!("hiii");
    Ok(())
}

fn setup_tracing() -> Result<()> {
    macro_rules! grimoire_crate {
        ($name:literal) => {
            if cfg!(debug_assertions) {
                concat!($name, "=debug")
            } else {
                concat!($name, "=info")
            }
            .parse()
        };
    }

    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(grimoire_crate!("grimoire")?)
        .add_directive(grimoire_crate!("grimoire-cli")?)
        .add_directive(grimoire_crate!("grimoire-web")?);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .try_init()?;

    Ok(())
}
