mod authentication;
mod common;
mod database;
mod schema;
mod users;

use actix_cors::Cors;
use actix_web::{
    get, http, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use authentication::middleware::AuthenticationCheck;
use common::openapi::ApiDoc;
use database::{
    model::db::DbPool,
    tools::{establish_db_connection, run_migrations},
};

use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub const JWT_ALGORITHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::HS256;
pub const JWT_LIFETIME: usize = 60 * 60 * 24 * 30; // 30 days
lazy_static! {
    static ref JWT_SECRET: Vec<u8> = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file")
        .into_bytes();
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(&JWT_SECRET);
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(&JWT_SECRET);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();
    let pool: DbPool = establish_db_connection();
    match run_migrations(pool.clone()) {
        Ok(_) => println!("Database schema updated."),
        Err(e) => println!("Error running migrations: {}", e),
    };

    HttpServer::new(move || {
        let cors = if cfg!(debug_assertions) {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_origin("https://example.com") // Replace with your allowed origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
        };
        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default().log_target("debug"))
            .wrap(middleware::Logger::new(
                "ip: %a user-agent: ${User-Agent}i time_to_complete: %D",
            ))
            // Set up the Swagger UI on /api/spec/
            .service(
                SwaggerUi::new("/spec/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // redirects /spec to /spec/
            .service(web::redirect("/spec", "/spec/"))
            // Register the user routes
            .service(
                web::scope("/api")
                    .wrap(AuthenticationCheck)
                    .configure(users::routes::config),
            )
            // Register the authentication routes
            .service(web::scope("/auth").configure(authentication::routes::config))
            // Register the database routes
            .service(web::scope("/admin").configure(database::routes::config))
            // Simple health check for /
            .service(hello)
    })
    .workers(1)
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}
