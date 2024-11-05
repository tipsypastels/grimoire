use grimoire_core::Grimoire as Core;
use std::sync::Arc;
use tokio::sync::{self, RwLock};

pub type Page<'a> = sync::RwLockReadGuard<'a, Core>;
pub type PageMut<'a> = sync::RwLockWriteGuard<'a, Core>;

#[derive(Debug, Clone)]
pub struct Grimoire {
    core: Arc<RwLock<Core>>,
}

impl Grimoire {
    pub fn new(core: Core) -> Self {
        Self {
            core: Arc::new(RwLock::new(core)),
        }
    }

    pub async fn read(&self) -> Page<'_> {
        self.core.read().await
    }

    pub async fn write(&self) -> PageMut<'_> {
        self.core.write().await
    }
}
