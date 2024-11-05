use anyhow::{ensure, Result};
use camino::Utf8Path;
use tokio::process::Command;

pub async fn build(out_dir: &Utf8Path) -> Result<()> {
    let mut command = Command::new("rsync"); // cp -r, but faster
    let status = command
        .arg("-r")
        .arg("public/")
        .arg(out_dir)
        .status()
        .await?;

    ensure!(status.success(), "copying public failed, status {status}");
    Ok(())
}
