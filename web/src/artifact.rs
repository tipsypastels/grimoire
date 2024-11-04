use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use std::sync::LazyLock;
use tokio::fs;

static PROJECT: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com", "tipsypastels", "grimoire")
        .expect("could not create artifact directory")
});

pub static BIN_DIR: LazyLock<Box<Utf8Path>> = LazyLock::new(|| {
    Utf8PathBuf::try_from(PROJECT.cache_dir().join("bin"))
        .expect("BIN_DIR not UTF-8")
        .into()
});

pub async fn init() -> Result<()> {
    fs::create_dir_all(BIN_DIR.as_ref()).await?;
    Ok(())
}
