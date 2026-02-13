use crate::models::templates::{CreateTemplate, TodosTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn create_handler() -> Response {
    let create_template = CreateTemplate {}.render().unwrap();

    Html(create_template).into_response()
}

pub async fn todos_handler() -> Response {
    let todos_template = TodosTemplate {}.render().unwrap();

    Html(todos_template).into_response()
}
