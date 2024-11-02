use std::sync::Mutex;

use actix_web::{guard, post, web, App, HttpResponse, HttpServer, Responder};

mod scope_one;

struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<u32>,
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
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/scope-one")
                    .guard(guard::Header("custom-header", "valid"))
                    .configure(scope_one::config),
            )
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
