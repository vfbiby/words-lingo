mod hello_routes;
mod works_routes;
mod words_routes;

use ntex::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    hello_routes::configure(cfg);
    works_routes::configure(cfg);
    words_routes::configure(cfg);
}