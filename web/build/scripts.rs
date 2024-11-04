use crate::binary::{ESBUILD, PNPM};
use anyhow::{ensure, Context, Result};
use camino::Utf8Path;
use tokio::{fs, process::Command};

pub async fn build(out_dir: &Utf8Path) -> Result<()> {
    pnpm().await.context("pnpm error")?;
    esbuild(out_dir).await.context("esbuild error")?;
    Ok(())
}

async fn pnpm() -> Result<()> {
    if fs::try_exists("node_modules").await? {
        return Ok(());
    }

    let binary = PNPM.path().await?;
    let mut command = Command::new(binary);
    let status = command.kill_on_drop(true).arg("install").status().await?;
    let success = status.success();

    ensure!(success, "pnpm failed, status {status}");
    Ok(())
}

async fn esbuild(out_dir: &Utf8Path) -> Result<()> {
    let output = out_dir.join("js");
    let binary = ESBUILD.path().await?;
    let mut command = Command::new(binary);

    command
        .kill_on_drop(true)
        .arg("js/packs/**/*.ts")
        .arg("--bundle")
        .arg("--minify")
        .arg("--tsconfig=tsconfig.json")
        .arg(format!("--outdir={output}"));

    let status = command.status().await?;
    let success = status.success();

    ensure!(success, "esbuild error, status {status}");
    Ok(())
}
