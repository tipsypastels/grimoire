use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use futures::{
    stream::{self, FuturesUnordered},
    Stream, StreamExt,
};
use std::future::Future;

pub fn walk_dir(path: &Utf8Path) -> impl Stream<Item = Utf8PathBuf> + Send + 'static {
    async fn read(path: Utf8PathBuf, stack: &mut Vec<Utf8PathBuf>) -> Vec<Utf8PathBuf> {
        let Ok(mut dir) = tokio::fs::read_dir(&path).await else {
            return Vec::new();
        };

        let mut files = Vec::new();

        while let Some(entry) = dir.next_entry().await.transpose() {
            let Ok(entry) = entry else {
                continue;
            };
            let Ok(metadata) = entry.metadata().await else {
                continue;
            };
            let Ok(path) = Utf8PathBuf::from_path_buf(entry.path()) else {
                continue;
            };
            if metadata.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
        files
    }

    stream::unfold(vec![path.to_owned()], |mut stack| async {
        let path = stack.pop()?;
        let files = stream::iter(read(path, &mut stack).await);
        Some((files, stack))
    })
    .flatten()
}

pub async fn walk_dir_and_read(
    path: &Utf8Path,
) -> FuturesUnordered<impl Future<Output = Result<(Utf8PathBuf, String)>>> {
    walk_dir(path)
        .filter(|p| {
            let is_file = p.is_file();
            async move { is_file }
        })
        .map(|path| async move {
            let text = read_to_string(&path).await?;
            anyhow::Ok((path, text))
        })
        .collect::<FuturesUnordered<_>>()
        .await
}

pub async fn read_to_string(path: &Utf8Path) -> Result<String> {
    tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("failed to read {path}"))
}
