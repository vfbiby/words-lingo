use dotenv::dotenv;
use ntex::web::{App, HttpServer};
use sea_orm::Database;
use words_lingo::routes;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 创建数据库连接
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    // 创建 web 服务
    HttpServer::new(move || {
        App::new()
            // 添加数据库连接状态
            .state(db.clone())
            // 注册路由
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?  // 绑定到本地8080端口
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntex::http::StatusCode;
    use ntex::web::test;

    #[ntex::test]
    async fn get_root_response_with_hello_world() {
        // Given: a web service with a root endpoint
        let mut app = test::init_service(App::new().configure(routes::configure)).await;

        // When: a GET request is made to the root endpoint
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Then: the response status should be OK and the body should be "Hello World!"
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
    }

    #[ntex::test]
    async fn get_works_response_with_it_works() {
        // Given: a web service with a /works endpoint
        let mut app = test::init_service(App::new().configure(routes::configure)).await;

        // When: a GET request is made to the /works endpoint
        let req = test::TestRequest::get().uri("/works").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Then: the response status should be OK and the body should be "it works"
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(body, "it works");
    }

    #[ntex::test]
    async fn get_not_found_response_not_found_status_code() {
        // Given: a web service with no /not_found endpoint
        let mut app = test::init_service(App::new().configure(routes::configure)).await;

        // When: a GET request is made to a non-existent endpoint
        let req = test::TestRequest::get().uri("/not_found").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Then: the response status should be NOT FOUND
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[ntex::test]
    async fn get_works_with_query_params() {
        // Given: a web service with a /works endpoint
        let mut app = test::init_service(App::new().configure(routes::configure)).await;

        // When: a GET request is made to the /works endpoint with query parameters
        let req = test::TestRequest::get().uri("/works?param=value").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Then: the response status should be OK and the body should be "it works"
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(body, "it works");
    }

    #[ntex::test]
    async fn get_hello_response_with_hello() {
        // Given: a web service with a /hello endpoint
        let mut app = test::init_service(App::new().configure(routes::configure)).await;

        // When: a GET request is made to the /hello endpoint
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Then: the response status should be OK and the body should be "hello"
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(body, "hello");
    }
}