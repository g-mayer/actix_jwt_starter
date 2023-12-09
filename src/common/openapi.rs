use crate::authentication::model::LoginRequest;
use crate::authentication::routes as authentication;
use crate::common::model::AppError;
use crate::database::model::users::UserRole;
use crate::database::model::users::{CreateUserRequest, UpdateUserRequest, User};
use crate::database::routes as database;
use crate::users::routes as users;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Authentication handlers
        authentication::login_handler,
        // User handlers
        users::find_all_users_handler,
        users::find_user_handler,
        users::create_user_handler,
        users::update_user_handler,
        users::delete_user_handler,
        // Database handlers
        database::seed_database_handler
    ),
    components(
        schemas(UpdateUserRequest, CreateUserRequest, User, LoginRequest, UserRole),
        responses(User, AppError),
    ),
    info(
        title = "Rust API",
        description = "API documentation for Actix-Web API",
        contact(
            name = "Developer",
            email = "support@project.com",
            url =  "https://project.com/contact",
        ),
    ),
    tags(
        (name = "authentication", description = "Authentication endpoints."),
        (name = "users", description = "User management endpoints."),
        (name = "database", description = "Database management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]

pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Check if components are already defined
        if let Some(components) = &mut openapi.components {
            // Add the security scheme to the existing components
            components.add_security_scheme(
                "token_jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        } else {
            // If components are not defined, create new components with the security scheme
            openapi.components = Some(
                utoipa::openapi::ComponentsBuilder::new()
                    .security_scheme(
                        "token_jwt",
                        SecurityScheme::Http(
                            HttpBuilder::new()
                                .scheme(HttpAuthScheme::Bearer)
                                .bearer_format("JWT")
                                .build(),
                        ),
                    )
                    .build(),
            );
        }
    }
}
