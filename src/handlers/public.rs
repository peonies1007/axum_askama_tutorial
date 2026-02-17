use crate::{handlers::errors::AppError, models::templates::HomeTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn home() -> Result<Response, AppError> {
    let home_template = HomeTemplate {}.render().unwrap();

    Ok(Html(home_template).into_response())
}
