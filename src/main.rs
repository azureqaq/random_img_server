use anyhow::Result;
use img::config::ConfigFile;
use img::server::server;

#[tokio::main]
async fn main() -> Result<()> {
    // 命令行: img [<CONFIG_PATH>]
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or("./config.toml".to_string());

    // 如果无法获取配置文件，则使用默认值
    let config = ConfigFile::new_from_file(config_path).unwrap_or_default();

    // 开启服务
    server(config).await?;
    Ok(())
}
