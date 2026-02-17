use crate::{
    handlers::errors::AppError,
    models::templates::{CreateTemplate, TodosTemplate},
};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn create_handler() -> Result<Response, AppError> {
    let create_template = CreateTemplate {}.render().unwrap();

    Ok(Html(create_template).into_response())
}

pub async fn todos_handler() -> Result<Response, AppError> {
    let todos_template = TodosTemplate {}.render().unwrap();

    Ok(Html(todos_template).into_response())
}
