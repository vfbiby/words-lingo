use ntex::web::{self, App, HttpServer};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // 创建 web 服务
    web::HttpServer::new(|| {
        web::App::new()
            // 注册路由
            .service(web::resource("/").to(|| async { "Hello World!" }))
            // 添加新的api接口
            .service(web::resource("/works").to(|| async { "it works" }))
    })
    .bind("127.0.0.1:8080")?  // 绑定到本地8080端口
    .run()
    .await
}