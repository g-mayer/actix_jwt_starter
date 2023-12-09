use crate::database::model::users::{
    CreateUserDb, CreateUserRequest, UpdateUserDb, UpdateUserRequest, User,
};
use crate::schema::users as users_schema;
use crate::schema::users::{self, dsl::*};

use bcrypt::{hash, DEFAULT_COST};
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::{pg::PgConnection, result::QueryResult, OptionalExtension, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn create_user(conn: &mut PgConnection, user_data: CreateUserRequest) -> QueryResult<User> {
    let hash = match hash(user_data.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return Err(Error::QueryBuilderError("Password hashing failed".into())),
    };
    let new_user = CreateUserDb {
        username: user_data.username,
        email: user_data.email,
        hashed_password: hash,
        timezone: user_data.timezone,
        role: user_data.role as i32,
    };
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn find_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Option<User>> {
    users.find(user_id).first(conn).optional()
}

pub fn find_user_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<Option<User>> {
    users
        .filter(users_schema::email.eq(user_email))
        .first::<User>(conn)
        .optional()
}

pub fn find_user_by_username(
    conn: &mut PgConnection,
    user_username: &str,
) -> QueryResult<Option<User>> {
    users
        .filter(users_schema::username.eq(user_username))
        .first::<User>(conn)
        .optional()
}

pub fn find_all_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn update_user(
    conn: &mut PgConnection,
    user_id: Uuid,
    user_data: UpdateUserRequest,
) -> QueryResult<User> {
    let user_update = UpdateUserDb {
        username: user_data.username.clone(),
        hashed_password: match &user_data.password {
            Some(password) => match hash(password, DEFAULT_COST) {
                Ok(hashed) => Some(hashed),
                Err(_) => return Err(Error::QueryBuilderError("Password hashing failed".into())),
            },
            None => None,
        },
        email: user_data.email.clone(),
        timezone: user_data.timezone.clone(),
        role: match &user_data.role {
            Some(new_role) => Some(*new_role as i32),
            None => None,
        },
    };

    diesel::update(users.find(user_id))
        .set(&user_update)
        .get_result(conn)
}

pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}
