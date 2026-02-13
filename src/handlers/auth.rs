use crate::models::templates::{LoginTemplate, SignupTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn login_handler() -> Response {
    let login_template = LoginTemplate {}.render().unwrap();

    Html(login_template).into_response()
}

pub async fn signup_handler() -> Response {
    let signup_template = SignupTemplate {}.render().unwrap();

    Html(signup_template).into_response()
}
