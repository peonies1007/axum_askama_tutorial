use crate::{
    handlers::helpers,
    models::{
        templates::{LoginTemplate, SignupTemplate},
        user_form_model::AuthFormModel,
    },
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use validator::Validate;

pub async fn login_handler() -> Response {
    let login_template = LoginTemplate {}.render().unwrap();

    Html(login_template).into_response()
}

pub async fn signup_handler() -> Response {
    let signup_template = SignupTemplate {
        email: "",
        email_error: "",
        password_error: "",
    }
    .render()
    .unwrap();

    Html(signup_template).into_response()
}

pub async fn post_sign_up_handler(Form(user_form): Form<AuthFormModel>) -> Response {
    match user_form.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(errs) => {
            let errs = errs.to_string();

            let errs = errs.to_string();

            let mut email_error = String::new();
            let mut password_error = String::new();

            helpers::extract_error(&errs, |field, message| {
                if field == "email" {
                    email_error = message;
                } else if field == "password" {
                    password_error = message;
                }
            });

            let html_string = SignupTemplate {
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
            }
            .render()
            .unwrap();

            let response = Html(html_string).into_response();

            (StatusCode::BAD_REQUEST, response).into_response()
        }
    }
}
