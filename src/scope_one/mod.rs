use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::AppStateWithCounter;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(hello);
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
struct Counter {
    message: String,
    counter: u32,
}

#[utoipa::path(tag = "counter", 
    responses(
        (status = 201, description = "Todo created successfully", body = Counter),
        (status = 404, description = "Invalid custom-header")
    ),
    params(
        ("custom-header" = String, Header, description = "Set as valid to count")
    )
)]
#[get("/")]
async fn hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().json(Counter{
        counter: *counter,
        message: format!("Hello {app_name}!\nRequest number: {counter}")
    })
}
