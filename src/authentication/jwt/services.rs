use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    authentication::model::{Claims, LoginRequest},
    database::{
        model::db::DbPool,
        model::users::{User, UserRole},
        tools::get_connection,
    },
    users::service::find_user_by_email,
    ENCODING_KEY, JWT_ALGORITHM, JWT_LIFETIME, JWT_SECRET,
};
use actix_web::{dev::ServiceRequest, web};

use bcrypt::verify;
use jsonwebtoken::{decode, encode, DecodingKey, Header};
use log::error;

pub fn generate_token(user_id: &str, role: UserRole) -> String {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            return String::new();
        }
    };

    let expiration_time = now as usize + (JWT_LIFETIME); // Current time + 30 days

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration_time,
        role,
    };

    encode(&Header::default(), &claims, &ENCODING_KEY).unwrap()
}

pub fn validate_token(req: &ServiceRequest) -> Result<Claims, String> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or("No Authorization header")?
        .to_str()
        .map_err(|_| "Invalid Authorization header")?
        .trim_start_matches("Bearer ")
        .to_owned();

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&JWT_SECRET),
        &jsonwebtoken::Validation::new(JWT_ALGORITHM),
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| {
        error!("{:?}", e);
        "Invalid token".to_owned()
    });

    claims
}

pub fn verify_login_credentials(
    pool: web::Data<DbPool>,
    login_data: LoginRequest,
) -> Result<User, String> {
    let mut conn = get_connection(pool);
    match find_user_by_email(&mut conn, &login_data.email) {
        Ok(Some(user)) => match verify(&login_data.password, &user.hashed_password) {
            Ok(_valid) => Ok(user),
            Err(_) => Err("Password verification failed".into()),
        },
        Ok(None) => Err("User not found".into()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}
