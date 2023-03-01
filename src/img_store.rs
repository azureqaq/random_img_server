//! ## 索引 & 读取本地图片
//! - Img 记录其在文件系统中的路径，并提供读取到 Bytes 的方法
//! - ImgStore 用 { usize: Img } 储存图片路径
//!
//! ### 注意：
//! **每次启动不能保证图片顺序一致**

use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, Result};
use bytes::Bytes;
use std::path::{Path, PathBuf};
use tokio::fs::File as tokioFile;
use tokio::io::AsyncReadExt;

/// 图片类型
///
/// 目前仅支持 JPG，此部分为了以后升级准备
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ImageType {
    Jpg,
    Png,
}

#[derive(Clone)]
pub struct Image {
    pub img_type: ImageType,
    pub path: PathBuf,
}

impl Image {
    /// 读取
    pub async fn get_bytes(self) -> Result<Bytes> {
        let mut f = tokioFile::open(self.path.as_path()).await?;
        let mut buffer = Vec::new();
        let _n = f.read_to_end(&mut buffer).await?;
        let b = Bytes::from(buffer);
        Ok(b)
    }
}

pub struct ImageStore(AHashMap<usize, Image>);

impl ImageStore {
    /// 获取一个文件夹的图片索引
    fn get_img_from_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
        let mut res = Vec::new();
        let dir = std::fs::read_dir(dir)?;
        for p in dir {
            let Ok(p) = p else {
                continue;
            };
            let pp = p.path();
            if !pp.is_file() {
                continue;
            }

            let Some(extension) = pp.extension().and_then(|s| s.to_str() ).map(|s|{
                ["jpg", "jpeg"].contains(&s.to_lowercase().as_str())
            }) else {
                continue;
            };

            if !extension {
                continue;
            }

            res.push(pp);
        }
        Ok(res)
    }

    /// 读取 dirs 下的所有图片
    pub fn new_from_dirs<P: AsRef<Path>>(dirs: Vec<P>) -> Result<Self> {
        // 先去重
        let paths: AHashSet<PathBuf> = dirs.into_iter().map(|p| p.as_ref().to_path_buf()).collect();
        let mut res_paths = Vec::new();
        paths
            .into_iter()
            .for_each(|p| match Self::get_img_from_dir(&p) {
                Err(e) => {
                    eprintln!("加入文件夹 {} 失败, Error: {}", p.display(), e);
                }
                Ok(mut img_paths) => {
                    println!("加入文件夹: {}", p.display());
                    res_paths.append(&mut img_paths);
                }
            });
        println!("共加入 {} 个图片", res_paths.len());

        if res_paths.is_empty() {
            return Err(anyhow!("应该至少加入一个文件夹"));
        }

        // 转换类型
        let mut store = AHashMap::new();
        let mut id = 0;

        res_paths.into_iter().for_each(|path| {
            let img = Image {
                img_type: ImageType::Jpg,
                path,
            };
            store.insert(id, img);
            id += 1;
        });

        Ok(Self(store))
    }
}

impl std::ops::Deref for ImageStore {
    type Target = AHashMap<usize, Image>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod img_store_test {
    use super::*;

    #[test]
    fn load_img_test() {
        let store = ImageStore::new_from_dirs(vec!["./assets/imgs"]);
        assert!(store.is_ok());
        let store = store.unwrap();
        assert!(!store.is_empty());
        assert!(store.contains_key(&0));
        assert!(store.get(&0).is_some());
    }
}
