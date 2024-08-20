use crate::server::handler::calls::handler_calls_email;
use crate::server::handler::client_list::handler_status_client;
use crate::server::handler::single_client_call::handler_calls_email_one_client;
use crate::utils::create_table::create_table;
use axum::http::HeaderValue;
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Method;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn server_router() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url_connect: String = std::env::var("CLIENT_TABLE").unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&url_connect)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state: Arc<AppState> = Arc::new(AppState { db: pool.clone() });

    // ? Create table
    match create_table(&app_state.db).await {
        Ok(result) => {
            println!("✅ {}", result);
            true
        }
        Err(err) => {
            println!("🔥 Failed to create table: {:?}", err);
            false
        }
    };

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app: Router = Router::new()
        .route("/api/status", get(handler_status_client))
        .route("/api/calls", post(handler_calls_email))
        .route("/api/onecall", post(handler_calls_email_one_client))
        .with_state(app_state)
        .layer(cors);

    println!("Server Calibri started successfully at 0.0.0.0:8070");

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8070").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
