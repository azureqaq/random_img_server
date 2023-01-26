use anyhow::Result;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    if let Err(e) = mma().await {
        log::error!("Err: {}", e);
    }
}

async fn mma() -> Result<()> {
    Ok(())
}
