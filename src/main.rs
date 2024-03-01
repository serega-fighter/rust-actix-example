use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};
use std::sync::Mutex;
use std::time::Duration;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })))
            .route("/hey", web::get().to(manual_hello))
            .route("/", web::get().to(index))
    })
        .keep_alive(Duration::from_secs(5))
        .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}