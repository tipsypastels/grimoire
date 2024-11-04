use anyhow::Result;
use camino::Utf8PathBuf;
use std::env;

mod artifact;
mod binary;
mod tailwind;

#[tokio::main]
async fn main() -> Result<()> {
    artifact::init().await?;
    binary::init().await?;

    let out_dir: Utf8PathBuf = env::var("OUT_DIR").unwrap().into();

    tailwind::build(&out_dir).await?;

    Ok(())
}
