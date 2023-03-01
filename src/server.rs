//! ## API 接口
//!
//! 随机图片 + 指定ID图片，如果有错误返回 404 图片
//!
//! - GET http://host:port/random
//! - GET http://host:port/ID/pic.jpg

use crate::{config::ConfigFile, img_store::ImageStore};
use anyhow::Result;
use axum::{
    extract::{self, Extension},
    response::{ErrorResponse, Redirect},
    routing, Router,
};
use bytes::Bytes;
use chrono::prelude::*;
use rand::Rng;
use std::{net::SocketAddr, sync::Arc};

lazy_static::lazy_static! {
    static ref NOTFOUND: Bytes = {
        let img = include_bytes!("../assets/imgs/404.jpg");
        Bytes::from_static(img)
    };

    static ref OFFSET: FixedOffset = FixedOffset::east_opt(8*60*60).unwrap();
}

fn get_now() -> NaiveDateTime {
    let now = DateTime::<FixedOffset>::from_utc(Utc::now().naive_utc(), *OFFSET);
    now.naive_local()
}

/// 获取图片
async fn random_img(extract::Extension(store): Extension<Arc<ImageStore>>) -> Redirect {
    // 后去范围内随机id
    let mut rng = rand::thread_rng();
    let id: usize = rng.gen_range(0..store.len());
    let uri = format!("/{}/pic.jpg", id);
    Redirect::temporary(&uri)
}

/// 通过图片id获取图片
///
/// 如果不存在则返回 404 图片
async fn find_img_by_id(
    extract::Path(id): extract::Path<usize>,
    extract::Extension(store): Extension<Arc<ImageStore>>,
) -> axum::response::Result<Bytes> {
    let img = store
        .get(&id)
        .ok_or_else(|| {
            eprintln!("{} 未找到: {}", get_now().format("%Y-%m-%d %H:%M:%S"), id);
            ErrorResponse::from(NOTFOUND.clone())
        })?
        .clone();
    let res = img.get_bytes().await;
    match res {
        Err(_) => Err(ErrorResponse::from(NOTFOUND.clone())),
        Ok(b) => {
            println!("{} 获取ID：{}", get_now().format("%Y-%m-%d %H:%M:%S"), id);
            Ok(b)
        }
    }
}

pub async fn server(config: ConfigFile) -> Result<()> {
    let dirs = config.dirs.into_iter().collect();
    let img_store = Arc::new(ImageStore::new_from_dirs(dirs)?);
    let app = Router::new()
        .route("/random", routing::get(random_img))
        .route("/:id/pic.jpg", routing::get(find_img_by_id))
        .layer(Extension(img_store));
    let addr = SocketAddr::from((config.ip, config.port));

    println!("\n绑定到: {}\n", addr);
    axum::Server::try_bind(&addr)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
