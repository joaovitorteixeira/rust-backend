use std::sync::Mutex;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};

struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<u32>,
}

#[get("/")]
async fn hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Hello {app_name}!\nRequest number: {counter}"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
        app_name: String::from("My First Rust Backend"),
    });

    HttpServer::new(move || {
        // Now scope any endpoint under /scope-one/ need custom-header = valid
        let scope_one = web::scope("/scope-one")
            .guard(guard::Header("custom-header", "valid"))
            .service(hello);

        App::new()
            .app_data(app_state.clone())
            .service(scope_one)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
