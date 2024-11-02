use actix_web::{get, guard, web, HttpResponse, Responder};

use crate::AppStateWithCounter;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

#[get("/")]
async fn hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Hello {app_name}!\nRequest number: {counter}"))
}
