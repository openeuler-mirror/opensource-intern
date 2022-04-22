mod config;
mod error;
mod handler;
mod response;
use crate::{
    config::Config,
    handler::{index_handler, static_handler},
};
use axum::{
    handler::Handler,
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use dotenv::dotenv;
use tokio::signal;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "openEuler-cheat-sheet=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok(); // 解析 .env 文件
    let cfg = Config::from_env().expect("初始化项目配置失败");

    let app = Router::new()
        .route("/", get(index_handler))
        .nest(
            "/static",
            get_service(ServeDir::new("../front/public")).handle_error(|err| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("处理静态资源出错: {:?}", err),
                )
            }),
        )
        .fallback(static_handler.into_service())
        .layer(TraceLayer::new_for_http());

    let addr = cfg.web.addr;
    tracing::info!("服务器监听于：{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    println!("signal received, starting graceful shutdown");
}