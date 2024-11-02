use std::{env, net::Ipv4Addr};
use actix_web::{
    middleware::Logger,
    App,  HttpServer, 
};
use diesel::{Connection, SqliteConnection};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(        
        tags(
            (name = "backend-rust", description = "My first backend in rust")
        ),
    )]
    struct ApiDoc;

    establish_db_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .service(
                utoipa_actix_web::scope("/api/v1").configure(user::config)
            )
            .openapi_service(|api| Redoc::with_url("/redoc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .map(|app| app.service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc")))
            .into_app()
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

fn establish_db_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();

    let file_name = env::var("DATABASE_URL").expect("Database URL is missing");

    SqliteConnection::establish(&file_name).unwrap_or_else(|_| panic!("Error connection to {}", file_name))
}