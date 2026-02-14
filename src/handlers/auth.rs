use crate::models::{
    templates::{LoginTemplate, SignupTemplate},
    user_form_model::UserFormModel,
};
use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};

pub async fn login_handler() -> Response {
    let login_template = LoginTemplate {}.render().unwrap();

    Html(login_template).into_response()
}

pub async fn signup_handler() -> Response {
    let signup_template = SignupTemplate {}.render().unwrap();

    Html(signup_template).into_response()
}

pub async fn post_sign_up_handler(Form(user_form): Form<UserFormModel>) -> Response {
    tracing::info!(
        "Email is {} and Password is {}",
        user_form.email,
        user_form.password
    );
    Redirect::to("/").into_response()
}
