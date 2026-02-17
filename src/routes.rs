use std::time::Duration;

use crate::handlers::auth::{login_handler, post_sign_up_handler, signup_handler};
use crate::handlers::public::home;
use crate::handlers::todos::{create_handler, todos_handler};
use crate::models::app::AppState;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::{routing::get, Router};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn router(app_state: AppState) -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_handler))
        .route("/todos", get(todos_handler))
        .route("/log-in", get(login_handler))
        .route("/sign-up", get(signup_handler).post(post_sign_up_handler))
        .nest_service("/static", server_dir)
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        );

    app
}

fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "Request started method {} path {}",
        request.method(),
        request.uri().path()
    )
}
fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "Response generate status {} in {:?}",
        response.status(),
        latency
    )
}
fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("Request failed {:?} after {:?}", error, latency)
}
