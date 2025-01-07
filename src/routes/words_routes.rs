use ntex::web::{self, HttpResponse};

#[web::post("/words")]
async fn post_words() -> impl web::Responder {
    HttpResponse::Ok()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(post_words);
}