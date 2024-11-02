use std::{net::Ipv4Addr, sync::Mutex};

use actix_web::{
    guard,
    middleware::Logger,
    post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;
mod scope_one;

struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<u32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
struct Echo {
    message: String,
}

#[utoipa::path(tag = "echo", responses((status = 201, description = "Todo created successfully", body = Echo),))]
#[post("/echo")]
async fn echo(todo: Json<Echo>) -> impl Responder {
    HttpResponse::Ok().json(todo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(        
        tags(
            (name = "backend-rust", description = "My first backend in rust")
        ),
    )]
    struct ApiDoc;

    let app_state = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
        app_name: String::from("My First Rust Backend"),
    });

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .app_data(app_state.clone())
            .service(
                utoipa_actix_web::scope("/scope-one")
                    .guard(guard::Header("custom-header", "valid"))
                    .configure(scope_one::config),
            )
            .service(echo)
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
