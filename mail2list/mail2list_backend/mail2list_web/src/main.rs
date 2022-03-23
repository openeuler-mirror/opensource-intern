use std::time::Duration;

use axum::{
    Router, Server,
};
use mail2list_web::{
    config::log::init_log,
    routers::{maillist},
    MAIL2LIST_CONFIG,
};
use log::info;
use tower_http::cors::{Any, CorsLayer};



/**
 *method:main
 *desc:程序主入口方法 admin 管理端api api:小程序,h5,app使用
 *author:zhaorunqi
 *email:348040933QQ.com
 */
#[tokio::main]
async fn main() {
    init_log();
    info!(
        " - Local:   http://{}:{}",
        MAIL2LIST_CONFIG.server.host.replace("0.0.0.0", "127.0.0.1"),
        MAIL2LIST_CONFIG.server.port
    );

    let server = format!(
        "{}:{}",
        MAIL2LIST_CONFIG.server.host, MAIL2LIST_CONFIG.server.port
    );
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(60) * 10);

    //绑定端口 初始化 路由
    let app = Router::new()
        .nest(
            "/maillist",
            maillist::routers(),
        )
        .layer(cors);

    Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
