use crate::{
    authentication::jwt::services::generate_token,
    authentication::jwt::services::verify_login_credentials, common::model::AppError,
    database::model::db::DbPool,
};

use super::model::LoginRequest;
use actix_web::{post, web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_handler);
}

// sign in with email and password
#[utoipa::path(
    path = "/auth/login",
    request_body(
        content = LoginRequest,
        description = "Login credentials"
    ),
    responses(
        (status = 200, description = "Logged in user.", body = User),
        (status = 500, description = "Invalid credentials.")
    ),
    operation_id = "loginUser"
)]
#[post("/login")]
async fn login_handler(
    pool: web::Data<DbPool>,
    req_body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    match verify_login_credentials(pool, req_body.into_inner()) {
        Ok(user) => {
            let token = generate_token(&user.id.to_string(), user.role);

            Ok(HttpResponse::Ok()
                .append_header(("Authorization", format!("Bearer {}", token)))
                .json(user))
        }
        Err(_) => Err(AppError::UnauthorizedError(
            "Invalid credentials".to_string(),
        )),
    }
}
