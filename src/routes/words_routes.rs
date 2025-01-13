use crate::entity::word::Entity as Word;
use ntex::web::{self, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait};

#[web::get("/words")]
async fn get_words(db: web::types::State<DatabaseConnection>) -> impl web::Responder {
    match Word::find().all(db.get_ref()).await {
        Ok(words) => HttpResponse::Ok().json(&words),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[web::post("/words")]
async fn post_words() -> impl web::Responder {
    HttpResponse::Ok()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_words).service(post_words);
}
