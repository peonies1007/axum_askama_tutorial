use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn home() -> Response {
    let home_template = HomeTemplate {}.render().unwrap();

    Html(home_template).into_response()
}
