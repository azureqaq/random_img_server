use anyhow::Result;
use img::config::ConfigFile;
use img::server::server;
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
    // let args:Vec<String> = std::env::args().collect();
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./config.toml".to_string());

    let config = ConfigFile::new_from_file(config_path)?;

    server(config).await?;

    Ok(())
}
