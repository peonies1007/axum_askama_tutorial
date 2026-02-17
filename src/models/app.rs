use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub connection_db: PgPool,
}
