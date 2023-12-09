use std::error::Error;

use actix_web::web;
use diesel::{prelude::*, r2d2::ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::PooledConnection;

use super::model::db::DbPool;

pub fn establish_db_connection() -> DbPool {
    let database_url = dotenvy::var("DATABASE_URL").expect("Error: Missing database url.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_connection(
    pool: web::Data<DbPool>,
) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
    pool.get()
        .expect("Error: Couldn't get db connection from pool")
}

// This macro creates binaries of the migrations to run in a production environment
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(pool: DbPool) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = pool
        .get()
        .expect("Error: Couldn't get db connection from pool");

    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
