use grimoire_core::Grimoire;
use std::sync::Arc;
use tokio::sync::{self, RwLock};

pub type GrimoirePage<'a> = sync::RwLockReadGuard<'a, Grimoire>;
pub type GrimoirePageMut<'a> = sync::RwLockWriteGuard<'a, Grimoire>;

#[derive(Debug, Clone)]
pub struct GrimoireLock {
    grimoire: Arc<RwLock<Grimoire>>,
}

impl GrimoireLock {
    pub fn new(grimoire: Grimoire) -> Self {
        Self {
            grimoire: Arc::new(RwLock::new(grimoire)),
        }
    }

    pub async fn read(&self) -> GrimoirePage<'_> {
        self.grimoire.read().await
    }

    pub async fn write(&self) -> GrimoirePageMut<'_> {
        self.grimoire.write().await
    }
}
