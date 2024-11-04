use crate::binary::TAILWIND;
use anyhow::{ensure, Result};
use camino::Utf8Path;
use tokio::process::Command;

pub async fn build(out_dir: &Utf8Path) -> Result<()> {
    let output = out_dir.join("tailwind.css");
    let binary = TAILWIND.path().await?;
    let mut command = Command::new(binary.as_str());

    command
        .kill_on_drop(true)
        .args(["--config", "tailwind.config.js"])
        .args(["--input", "tailwind.css"])
        .args(["--output", output.as_str()])
        .arg("--minify");

    let status = command.status().await?;
    let success = status.success();

    ensure!(success, "tailwind failed, status {status}");
    Ok(())
}
