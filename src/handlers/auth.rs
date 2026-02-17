use crate::{
    data::{errors::DataError, user::create_user},
    models::{
        app::AppState,
        templates::{LoginTemplate, SignupTemplate},
        user_form_model::AuthFormModel,
    },
};

use super::{errors::AppError, helpers};

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use validator::Validate;

pub async fn login_handler() -> Result<Response, AppError> {
    let login_template = LoginTemplate {}.render()?;

    Ok(Html(login_template).into_response())
}

pub async fn signup_handler() -> Result<Response, AppError> {
    let signup_template = SignupTemplate {
        email: "",
        email_error: "",
        password_error: "",
    }
    .render()?;

    Ok(Html(signup_template).into_response())
}

pub async fn post_sign_up_handler(
    State(app_state): State<AppState>,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError> {
    match user_form.validate() {
        Ok(_) => {
            let result = create_user(
                &app_state.connection_db,
                &user_form.email,
                &user_form.password,
            )
            .await;

            if let Err(err) = result {
                if let DataError::FailedQuery(e) = err {
                    tracing::error!("Failed to sign up {}", e);
                    let html_string = SignupTemplate {
                        email: &user_form.email,
                        email_error: &e,
                        password_error: "",
                    }
                    .render()
                    .unwrap();

                    let response = Html(html_string).into_response();

                    return Ok((StatusCode::BAD_REQUEST, response).into_response());
                } else {
                    Err(err)?
                }
            }

            Ok(Redirect::to("/log-in").into_response())
        }
        Err(errs) => {
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

            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }
}
