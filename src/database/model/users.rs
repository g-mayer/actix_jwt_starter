use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::deserialize::FromSqlRow;
use diesel::sql_types::Integer;
use diesel::Queryable;
use diesel::{deserialize, prelude::*};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use crate::schema::users;

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, ToSchema, FromSqlRow, Copy, PartialOrd,
)]
pub enum UserRole {
    Admin = 2,
    User = 1,
    Guest = 0,
}

// Tell Diesel and Rust how to handle UserRole enum for querying or inserting into database.
impl<DB> FromSql<Integer, DB> for UserRole
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(UserRole::Guest),
            1 => Ok(UserRole::User),
            2 => Ok(UserRole::Admin),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, ToSchema, ToResponse, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub timezone: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug, ToSchema, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDb {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub timezone: String,
    pub role: i32,
}

#[derive(Deserialize, Debug, ToSchema, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub timezone: String,
    pub role: UserRole,
}

#[derive(Deserialize, Debug, ToSchema, Clone)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub timezone: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Deserialize, Debug, ToSchema, Clone, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDb {
    pub username: Option<String>,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    pub timezone: Option<String>,
    pub role: Option<i32>,
}
