use server::server::server_router;

mod models;
mod server;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = server_router().await.expect("Ошибка сервера");

    Ok(())
}
