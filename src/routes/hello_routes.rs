use ntex::web::{self, HttpResponse};

#[web::get("/")]
async fn root() -> impl web::Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[web::get("/hello")]
async fn greet() -> impl web::Responder {
    HttpResponse::Ok().body("hello")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(greet);
    cfg.service(root);
}