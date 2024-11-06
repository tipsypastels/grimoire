use axum::{extract::FromRequestParts, http::request::Parts};
use grimoire_core::Grimoire;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct App {
    pub grimoire: Grimoire,
}

impl App {
    pub fn new(grimoire: Grimoire) -> Self {
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
    grimoire: Grimoire,
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
