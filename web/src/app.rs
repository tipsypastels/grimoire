use crate::grimoire::GrimoireLock;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct App {
    pub grimoire: GrimoireLock,
}

impl App {
    pub fn new(grimoire: GrimoireLock) -> Self {
        Self { grimoire }
    }
}

#[axum::async_trait]
impl FromRequestParts<App> for App {
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut Parts, app: &App) -> Result<Self, Infallible> {
        Ok(app.clone())
    }
}

app_accessors! {
    grimoire: GrimoireLock,
}

macro_rules! app_accessors {
    ($($field:ident: $ty:ty),*$(,)?) => {
        $(
            #[axum::async_trait]
            impl FromRequestParts<App> for $ty {
                type Rejection = Infallible;

                async fn from_request_parts(_parts: &mut Parts, app: &App) -> Result<$ty, Infallible> {
                    Ok(app.$field.clone())
                }
            }
        )*
    };
}

use app_accessors;
