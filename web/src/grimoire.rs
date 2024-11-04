use grimoire_lib::Grimoire as Inner;
use std::sync::Arc;
use tokio::sync::{self, RwLock};

pub type Page<'a> = sync::RwLockReadGuard<'a, Inner>;
pub type PageMut<'a> = sync::RwLockWriteGuard<'a, Inner>;

#[derive(Debug, Clone)]
pub struct Grimoire {
    inner: Arc<RwLock<Inner>>,
}

impl Grimoire {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub async fn read(&self) -> Page<'_> {
        self.inner.read().await
    }

    pub async fn write(&self) -> PageMut<'_> {
        self.inner.write().await
    }
}
