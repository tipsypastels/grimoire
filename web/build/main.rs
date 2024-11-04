use anyhow::Result;
use camino::Utf8PathBuf;
use std::env;
use tokio::try_join;

mod artifact;
mod binary;
mod scripts;
mod styles;

#[tokio::main]
async fn main() -> Result<()> {
    artifact::init().await?;
    binary::init().await?;

    let out_dir: Utf8PathBuf = env::var("OUT_DIR").unwrap().into();

    let scripts = scripts::build(&out_dir);
    let styles = styles::build(&out_dir);

    try_join!(scripts, styles)?;
    Ok(())
}
