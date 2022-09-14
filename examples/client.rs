use std::env;

use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt};
use tonic_demo::client::Client;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let username = env::var("NAME")?;
    let mut client = Client::new(username).await;
    client.login().await?;
    client.get_message().await?;

    let mut lines = io::BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        if line == ":q" {
            break;
        }
        client.send_message("my_room", line).await?;
    }
    Ok(())
}
