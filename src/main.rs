use anyhow::Result;
use img::server;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    if let Err(e) = mma().await {
        log::error!("Err: {}", e);
    }
}

async fn mma() -> Result<()> {
    server::server(7878).await?;
    Ok(())
}
