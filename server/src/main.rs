use agent_trade::server;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting server...");
    server::start().await;
}
