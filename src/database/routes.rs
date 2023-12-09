use crate::{
    common::model::AppError,
    database::{model::db::DbPool, service::seed_database, tools::get_connection},
};
use actix_web::{get, web, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(seed_database_handler);
}

#[utoipa::path(
    path = "/admin/seed",
    responses(
        (status = 200, description = "Successfully added data to database.", body = String),
        (status = 403, description = "Access denied in production mode.", body = String),
        (status = 409, description = "Data may already be seeded in database.", body = String),
        (status = 500, description = "Internal Server Error")
    ),
    operation_id = "seedDatabase"
)]
#[get("/seed")]
async fn seed_database_handler(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let environment = std::env::var("ENVIRONMENT").unwrap();
    if environment != "development" {
        return Err(AppError::NotFoundError(
            "Access denied in production mode".to_string(),
        ));
    }
    let mut conn = get_connection(pool);
    let result = seed_database(&mut conn);
    match result {
        Ok(_) => Ok("Successfully added data to database. Try logging in with email: admin@admin.com, password: admin".to_string()),
        Err(e) => Err(AppError::DatabaseError(format!(
            "Double check database, data may already be seeded. \n Error: {}",
            e
        ))),
    }
}
