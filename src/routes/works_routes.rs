use ntex::web::{self, HttpResponse};

#[web::get("/works")]
async fn works() -> impl web::Responder {
    HttpResponse::Ok().body("it works")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(works);
}