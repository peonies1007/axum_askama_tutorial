use askama::Template;
use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create))
        .route("/todos", get(todos))
        .route("/login", get(login))
        .route("/signup", get(signup))
        .nest_service("/static", server_dir);

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Response {
    let home_template = HomeTemplate {}.render().unwrap();

    Html(home_template).into_response()
}

async fn create() -> Response {
    let create_template = CreateTemplate {}.render().unwrap();

    Html(create_template).into_response()
}

async fn todos() -> Response {
    let todos_template = TodosTemplate {}.render().unwrap();

    Html(todos_template).into_response()
}

async fn login() -> Response {
    let login_template = LoginTemplate {}.render().unwrap();

    Html(login_template).into_response()
}

async fn signup() -> Response {
    let signup_template = SignupTemplate {}.render().unwrap();

    Html(signup_template).into_response()
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "pages/create.html")]
struct CreateTemplate {}

#[derive(Template)]
#[template(path = "pages/todos.html")]
struct TodosTemplate {}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
struct LoginTemplate {}

#[derive(Template)]
#[template(path = "pages/sign-up.html")]
struct SignupTemplate {}
