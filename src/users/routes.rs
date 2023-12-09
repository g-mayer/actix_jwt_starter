use std::collections::HashMap;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use uuid::Uuid;

use crate::authentication::model::Claims;
use crate::authentication::service::authenticate_user_role;
use crate::common::model::AppError;
use crate::database::model::users::{CreateUserRequest, UpdateUserRequest};
use crate::database::{model::db::DbPool, tools::get_connection};
use crate::users::service::update_user;
use crate::users::service::{
    create_user, delete_user, find_all_users, find_user_by_email, find_user_by_id,
    find_user_by_username,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(find_user_handler);
    cfg.service(find_all_users_handler);

    cfg.service(create_user_handler);
    cfg.service(update_user_handler);
    cfg.service(delete_user_handler);
}

// Find user handler
#[utoipa::path(
    path = "/api/user",
    params(
        ("id" = Option<Uuid>, Query, description = "User ID"),
        ("email" = Option<String>, Query, description = "User email"),
        ("username" = Option<String>, Query, description = "User username")
    ),
    responses(
        (status = 200, description = "Successful response", body = User),
        (status = 401, description = "Missing or invalid authentication"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("token_jwt"=[])),
    operation_id = "findUser",
)]
#[get("/user")]
async fn find_user_handler(
    pool: web::Data<DbPool>,
    params: web::Query<HashMap<String, String>>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder, AppError> {
    authenticate_user_role(&claims)?;
    let mut conn = get_connection(pool);

    if let Some(user_id) = params.get("id") {
        match Uuid::parse_str(user_id) {
            Ok(id) => match find_user_by_id(&mut conn, id) {
                Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
                Ok(None) => Err(AppError::NotFoundError("User not found".to_string())),
                Err(_) => Err(AppError::DatabaseError("Internal Server Error".to_string())),
            },
            Err(_) => Err(AppError::ValidationError("Invalid user id".to_string())),
        }
    } else if let Some(email) = params.get("email") {
        match find_user_by_email(&mut conn, email) {
            Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
            Ok(None) => Err(AppError::NotFoundError("User not found".to_string())),
            Err(_) => Err(AppError::DatabaseError("Internal Server Error".to_string())),
        }
    } else if let Some(username) = params.get("username") {
        match find_user_by_username(&mut conn, username) {
            Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
            Ok(None) => Err(AppError::NotFoundError("User not found".to_string())),
            Err(_) => Err(AppError::DatabaseError("Internal Server Error".to_string())),
        }
    } else {
        Err(AppError::ValidationError(
            "Missing query parameters".to_string(),
        ))
    }
}

// Find all users handler
#[utoipa::path(
    path = "/api/users",
    responses(
        (status = 200, description = "Successful response", body = Vec<User>),
        (status = 401, description = "Missing or invalid authentication"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("token_jwt"=[])),
    operation_id = "findAllUsers"
)]
#[get("/users")]
async fn find_all_users_handler(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder, AppError> {
    authenticate_user_role(&claims)?; // Perform authentication check

    let mut conn = get_connection(pool);

    match find_all_users(&mut conn) {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(_) => Err(AppError::DatabaseError("Internal Server Error".to_string())),
    }
}

// Create user handler
#[utoipa::path(
    path = "/api/user",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully", body = User),
        (status = 400, description = "User already exists"),
        (status = 401, description = "Missing or invalid authentication"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("token_jwt"=[])),
    operation_id = "createUser")
]
#[post("/user")]
async fn create_user_handler(
    pool: web::Data<DbPool>,
    req_body: web::Json<CreateUserRequest>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder, AppError> {
    authenticate_user_role(&claims)?;
    let mut conn = get_connection(pool);

    match create_user(&mut conn, req_body.into_inner()) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err(AppError::ValidationError("User already exists".to_string()))
        }

        Err(_) => Err(AppError::DatabaseError(
            "Database error creating user".to_string(),
        )),
    }
}

// Update User Handler
#[utoipa::path(
    path = "/api/user/{user_id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 401, description = "Missing or invalid authentication"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("token_jwt"=[])),
    operation_id = "updateUserDetails"
)]
#[put("/user/{user_id}")]
async fn update_user_handler(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    req_body: web::Json<UpdateUserRequest>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder, AppError> {
    authenticate_user_role(&claims)?;

    let mut conn = get_connection(pool);

    match update_user(&mut conn, *user_id, req_body.into_inner()) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(AppError::DatabaseError("Internal Server Error".to_string())),
    }
}

// Delete User Handler
#[utoipa::path(
    path = "/api/user/{user_id}",
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 401, description = "Missing or invalid authentication"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("token_jwt"=[])),
    operation_id = "deleteUserById"
)]
#[delete("/user/{user_id}")]
async fn delete_user_handler(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    claims: web::ReqData<Claims>,
) -> Result<impl Responder, AppError> {
    authenticate_user_role(&claims)?;

    let mut conn = get_connection(pool);

    match delete_user(&mut conn, *user_id) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(AppError::DatabaseError("Internal Server Error".to_string())),
    }
}
