use anyhow::Error;
use askama_axum::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::borrow::Cow;

pub type ServeResult<T> = Result<T, ServeError>;

#[derive(Debug)]
pub enum ServeError {
    NotFound(Option<Cow<'static, str>>),
    InternalServerError(Error),
}

impl<E> From<E> for ServeError
where
    Error: From<E>,
{
    fn from(error: E) -> Self {
        Self::InternalServerError(error.into())
    }
}

impl IntoResponse for ServeError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound(_) => {
                #[derive(Template)]
                #[template(path = "notfound.html")]
                struct NotFoundTemplate {}

                let template = NotFoundTemplate {};
                (StatusCode::NOT_FOUND, template).into_response()
            }
            Self::InternalServerError(error) => {
                #[derive(Template)]
                #[template(path = "error.html")]
                struct ErrorTemplate {
                    error: String,
                }

                let error = format!("{error:?}");
                let template = ErrorTemplate { error };

                (StatusCode::INTERNAL_SERVER_ERROR, template).into_response()
            }
        }
    }
}

pub trait OrNotFound<T> {
    fn or_not_found(self) -> ServeResult<T>;
    fn or_not_found_msg(self, msg: impl Into<Cow<'static, str>>) -> ServeResult<T>;
}

impl<T> OrNotFound<T> for Option<T> {
    fn or_not_found(self) -> ServeResult<T> {
        match self {
            Self::Some(value) => Ok(value),
            Self::None => Err(ServeError::NotFound(None)),
        }
    }

    fn or_not_found_msg(self, msg: impl Into<Cow<'static, str>>) -> ServeResult<T> {
        match self {
            Self::Some(value) => Ok(value),
            Self::None => Err(ServeError::NotFound(Some(msg.into()))),
        }
    }
}
