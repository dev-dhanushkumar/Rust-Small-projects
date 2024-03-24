mod handler;
mod model;
mod schema;
mod route;


use std::sync::Arc;
use axum::{http::{header::CONTENT_TYPE, Method}, response::IntoResponse, Json};
use dotenv::dotenv;

use route::create_router;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use tower_http::cors::{Any, CorsLayer};


pub struct  AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");
    
    // let database_url="mysql://root:123hitesh@127.0.0.1:3306/rust_api"; 
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool= match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    // let app = Router::new()
    //     .route("/api/notes", get(get_notes))
    //     .with_state(Arc::new(AppState {db: pool.clone()}));

    let app = create_router(Arc::new(AppState {db: pool.clone()})).layer(cors);

    // println!("Server started successfully at 0.0.0.0:8080");
    println!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_notes() -> impl IntoResponse {
    const MESSAGE: &str = "Rama9";

    let json_response = serde_json::json!({
        "status": "ok",
        "message":MESSAGE
    });
    
    Json(json_response)
}