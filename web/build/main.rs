use anyhow::Result;
use camino::Utf8PathBuf;
use std::env;
use tokio::try_join;

mod artifact;
mod binary;
mod public;
mod scripts;
mod styles;

#[tokio::main]
async fn main() -> Result<()> {
    artifact::init().await?;
    binary::init().await?;

    let out_dir: Utf8PathBuf = env::var("OUT_DIR").unwrap().into();
    let public_dir = out_dir.join("public");

    let public = public::build(&public_dir);
    let scripts = scripts::build(&public_dir);
    let styles = styles::build(&public_dir);

    try_join!(public, scripts, styles)?;
    Ok(())
}
