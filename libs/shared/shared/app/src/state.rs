use sea_orm_migration::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}