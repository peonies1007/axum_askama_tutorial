use crate::handlers::auth::{login_handler, signup_handler};
use crate::handlers::public::home;
use crate::handlers::todos::{create_handler, todos_handler};
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_handler))
        .route("/todos", get(todos_handler))
        .route("/login", get(login_handler))
        .route("/signup", get(signup_handler))
        .nest_service("/static", server_dir);

    app
}
