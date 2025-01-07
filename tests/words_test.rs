#[cfg(test)]
mod tests {
    use words_lingo::routes;
    use ntex::http::StatusCode;
    use ntex::web::test::{self};
    use ntex::web::App;

    #[ntex::test]
    async fn post_words_should_response_ok() {
        let mut app = test::init_service(App::new().configure(routes::configure)).await;

        let request = test::TestRequest::post().uri("/words").to_request();
        let response = test::call_service(&mut app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
