use crate::{common::model::AppError, database::model::users::UserRole};

use super::model::Claims;

fn authenticate_claims(claims: &Claims, required_role: &UserRole) -> Result<bool, AppError> {
    if claims.role < *required_role {
        return Err(AppError::UnauthorizedError(
            "Missing or invalid authentication".to_string(),
        ));
    } else {
        return Ok(true);
    }
}

pub fn authenticate_user_role(claims: &Claims) -> Result<bool, AppError> {
    authenticate_claims(claims, &UserRole::User)
}
