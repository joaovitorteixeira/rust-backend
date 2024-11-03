use std::net::Ipv4Addr;
use actix_web::{
    middleware::Logger,
    App,  HttpServer, 
};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod user;
mod api_error;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    db::init();

    #[derive(OpenApi)]
    #[openapi(        
        tags(
            (name = "backend-rust", description = "My first backend in rust")
        ),
    )]
    struct ApiDoc;

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

