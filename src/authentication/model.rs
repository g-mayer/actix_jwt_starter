use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::model::users::UserRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
