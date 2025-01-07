// src/routes.rs
use ntex::web::{self, HttpResponse};

#[web::get("/")]
async fn hello() -> impl web::Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[web::get("/works")]
async fn works() -> impl web::Responder {
    HttpResponse::Ok().body("it works")
}

#[web::get("/hello")]
async fn greet() -> impl web::Responder {
    HttpResponse::Ok().body("hello")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(works)
       .service(greet);
}