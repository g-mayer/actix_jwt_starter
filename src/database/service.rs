use diesel::{PgConnection, QueryResult};

use crate::{database::model::users::CreateUserRequest, users::service::create_user};

use super::model::users::{User, UserRole};

pub fn seed_database(conn: &mut PgConnection) -> QueryResult<User> {
    let user_data = CreateUserRequest {
        username: "admin".to_string(),
        email: "admin@admin.com".to_string(),
        password: "admin".to_string(),
        timezone: "America/Los_Angeles".to_string(),
        role: UserRole::User,
    };

    create_user(conn, user_data)
}
