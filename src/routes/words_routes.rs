use ntex::web::{self, HttpResponse};

#[web::post("/words")]
async fn words() -> impl web::Responder {
    HttpResponse::Ok().body("words")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(words);
}