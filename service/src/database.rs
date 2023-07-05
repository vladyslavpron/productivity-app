use crate::entity::*;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema};

static DATABASE_URL: &str = "sqlite://data.db?mode=rwc";

pub async fn setup_database() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(DATABASE_URL.to_string());
    opt.sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await.unwrap();
    info!("Database connected");

    create_db_tables(&db).await;

    info!("Database tables created if they were not present");

    db
}

pub async fn create_db_tables(db: &DatabaseConnection) {
    let builder = db.get_database_backend();

    let stmt = builder.build(
        Schema::new(DbBackend::Sqlite)
            .create_table_from_entity(event::Entity)
            .if_not_exists(),
    );

    let stmt2 = builder.build(
        Schema::new(DbBackend::Sqlite)
            .create_table_from_entity(session::Entity)
            .if_not_exists(),
    );

    db.execute(stmt).await.unwrap();
    db.execute(stmt2).await.unwrap();
}
