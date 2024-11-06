mod error;
mod globals;

pub use error::{OrNotFound, ServeError, ServeResult};
pub use globals::Globals;

macro_rules! fa {
    ($name:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fa/", $name, ".svg"))
    };
}

pub(crate) use fa;
