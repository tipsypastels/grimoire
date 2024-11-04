use self::{
    unpack::Unpack,
    unzip::{Unzip, Unzipper},
};
use crate::artifact::BIN_DIR;
use anyhow::{Context, Result};
use camino::Utf8Path;
use futures::StreamExt;
use std::env::consts::{ARCH, OS};
use tokio::{fs, sync::OnceCell};
use url::Url;

mod unpack;
mod unzip;

pub async fn init() -> Result<()> {
    let tailwind = TAILWIND.state().await?;
    let esbuild = ESBUILD.state().await?;
    let pnpm = PNPM.state().await?;

    if tailwind || esbuild || pnpm {
        tracing::info!("binary installation complete")
    }

    Ok(())
}

pub static TAILWIND: Source<unpack::None, unzip::None> = Source {
    name: "tailwind",
    url: "https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.4/{TARGET}",
    cell: OnceCell::const_new(),
    target: |arch, os| match (arch, os) {
        ("x86_64", "macos") => Some("tailwindcss-macos-x64"),
        _ => None,
    },
    unpack: unpack::None,
    unzip: unzip::None { set_mode: 0o755 },
};

pub static ESBUILD: Source<unpack::Dir, unzip::TarGz> = Source {
    name: "esbuild",
    url: "https://registry.npmjs.org/@esbuild/{TARGET}/-/{TARGET}-0.23.0.tgz",
    cell: OnceCell::const_new(),
    target: |arch, os| match (arch, os) {
        ("x86_64", "macos") => Some("darwin-x64"),
        _ => None,
    },
    unpack: unpack::Dir("package/bin/esbuild"),
    unzip: unzip::TarGz,
};

pub static PNPM: Source<unpack::None, unzip::None> = Source {
    name: "pnpm",
    url: "https://github.com/pnpm/pnpm/releases/download/v9.11.0/pnpm-{TARGET}",
    cell: OnceCell::const_new(),
    target: |arch, os| match (arch, os) {
        ("x86_64", "macos") => Some("macos-x64"),
        _ => None,
    },
    unpack: unpack::None,
    unzip: unzip::None { set_mode: 0o755 },
};

pub struct Source<P, Z> {
    name: &'static str,
    url: &'static str,
    cell: OnceCell<Box<Utf8Path>>,
    target: fn(&'static str, &'static str) -> Option<&'static str>,
    unpack: P,
    unzip: Z,
}

impl<P: Unpack, Z: Unzip> Source<P, Z> {
    pub async fn path(&self) -> Result<&Utf8Path> {
        self.init().await.map(|(p, _)| p)
    }

    async fn state(&self) -> Result<bool> {
        self.init().await.map(|(_, v)| v)
    }

    async fn init(&self) -> Result<(&Utf8Path, bool)> {
        let mut did = false;
        let path = self
            .cell
            .get_or_try_init(|| self.try_init(&mut did))
            .await?;

        Ok((path, did))
    }

    async fn try_init(&self, did_init: &mut bool) -> Result<Box<Utf8Path>> {
        let path = BIN_DIR.join(self.name);
        if !fs::try_exists(&path).await? {
            *did_init = true;
            self.download(&path)
                .await
                .with_context(|| format!("failed to download {} binary", self.name))?;

            tracing::info!(binary = self.name, %path, "installed");
        }
        Ok(Box::from(path))
    }

    async fn download(&self, path: &Utf8Path) -> Result<()> {
        let client = reqwest::Client::new();
        let url = self.url()?;

        tracing::info!(binary = self.name, "downloading");

        let res = client.get(url).send().await?;
        let res = res.error_for_status()?;

        let mut unzipper = self.unzip(path).await?;
        let mut stream = res.bytes_stream();

        while let Some(result) = stream.next().await {
            let chunk = result.context("invalid chunk")?;
            unzipper.add(&chunk).await.context("invalid write")?;
        }

        unzipper.end().await?;
        self.unpack(path).await?;

        Ok(())
    }

    fn url(&self) -> Result<Url> {
        let target = (self.target)(ARCH, OS).with_context(|| format!("no binary {ARCH}-{OS}"))?;
        let url = self.url.replace("{TARGET}", target);
        Url::parse(&url).context("invalid URL")
    }

    async fn unzip(&self, path: &Utf8Path) -> Result<Z::Unzipper> {
        self.unzip.unzip(path).await.context("failed to unzip")
    }

    async fn unpack(&self, path: &Utf8Path) -> Result<()> {
        self.unpack
            .unpack(self.name, path)
            .await
            .context("failed to unpack")
    }
}
