use anyhow::Error;
use askama_axum::Template;
use axum::response::{IntoResponse, Response};

pub type ServeResult<T> = Result<T, ServeError>;

#[derive(Debug)]
pub struct ServeError(Error);

impl<E> From<E> for ServeError
where
    Error: From<E>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for ServeError {
    fn into_response(self) -> Response {
        #[derive(Template)]
        #[template(path = "error.html")]
        struct ErrorHtml {
            error: String,
        }

        let error = format!("{:?}", self.0);
        let template = ErrorHtml { error };

        template.into_response()
    }
}
