// src/main.rs
mod db;
mod handlers;
mod models;

use axum::{extract::State, routing::{delete, get, post, put}, Router};
use db::DataBase;
use dotenvy::dotenv;
use handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo, AppState};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载 .env（如果存在）
    dotenv().ok();

    // 初始化 tracing 日志，方便调试
    tracing_subscriber::fmt::init();

    // 从环境变量读取 DATABASE_URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or env");
    // 连接池配置：最大连接数 5（可调）
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 包装为我们的 Database
    let db = DataBase::new(pool);
    let shared_db = Arc::new(db);

    // 构建路由
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
        // 将共享状态注入到所有 handler 中
        .with_state(shared_db.clone());

    // 绑定地址
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    tracing::info!("Listening on {}", bind_addr);
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    // 启动服务
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
