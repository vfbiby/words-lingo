// src/routes.rs
use ntex::web::{self, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Hello World!") })),
    )
    .service(
        web::resource("/works")
            .route(web::get().to(|| async { HttpResponse::Ok().body("it works") })),
    )
    .service(
        web::resource("/hello")
            .route(web::get().to(|| async { HttpResponse::Ok().body("hello") })),
    );
}