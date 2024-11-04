use askama_axum::Template;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexHtml;
    IndexHtml
}
