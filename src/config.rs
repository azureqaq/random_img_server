//! 只读配置文件

use ahash::AHashSet;
use anyhow::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct ConfigFile {
    /// 文件夹们
    pub dirs: AHashSet<PathBuf>,
    pub ip: [u8; 4],
    /// 端口
    pub port: u16,
}

impl ConfigFile {
    /// 从配置文件读取
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = &std::fs::read_to_string(path)?;
        let conf = toml::from_str(content)?;
        Ok(conf)
    }

    /// 从参数传入
    pub fn new<P: AsRef<Path>>(paths: Vec<P>, port: u16, ip: [u8; 4]) -> Self {
        let dirs = paths
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .collect();
        Self { dirs, port, ip }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        let mut set = AHashSet::new();
        set.insert(PathBuf::from("./imgs"));
        Self {
            dirs: set,
            port: 7878,
            ip: [0, 0, 0, 0],
        }
    }
}

#[cfg(test)]
mod config_test {
    use super::*;

    #[test]
    fn read_from_file_test() {
        assert!(ConfigFile::new_from_file("config_template.toml").is_ok());
    }
}
