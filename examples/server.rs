use anyhow::Result;
use tonic_demo::server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    server::start().await;
    Ok(())
}
