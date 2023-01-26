//! API 接口

use std::{net::SocketAddr, sync::Arc};

use crate::img_store::ImageStore;
use anyhow::Result;
use axum::{
    extract::{self, Extension},
    response::{ErrorResponse, Redirect},
    routing, Router,
};
use bytes::Bytes;
use rand::Rng;

lazy_static::lazy_static! {
    static ref NOTFOUND: Bytes = {
        let img = include_bytes!("../assets/imgs/404.jpg");
        Bytes::from_static(img)
    };
}

/// 获取图片
async fn random_img(extract::Extension(store): Extension<Arc<ImageStore>>) -> Redirect {
    // 后去范围内随机id
    let mut rng = rand::thread_rng();
    let id: usize = rng.gen_range(0..=store.len());
    let uri = format!("/{}", id);
    Redirect::temporary(&uri)
}

/// 通过图片id获取图片
async fn find_img_by_id(
    extract::Path(id): extract::Path<usize>,
    extract::Extension(store): Extension<Arc<ImageStore>>,
) -> axum::response::Result<Bytes> {
    log::info!("获取ID：{}", id);
    let img = store
        .get(&id)
        .ok_or_else(|| ErrorResponse::from(NOTFOUND.clone()))?
        .clone();
    let res = img.get_bytes().await;
    match res {
        Err(_) => Err(ErrorResponse::from(NOTFOUND.clone())),
        Ok(b) => Ok(b),
    }
}

pub async fn server(port: u16) -> Result<()> {
    let img_store = Arc::new(ImageStore::new_from_dirs(vec!["./imgs"])?);
    let app = Router::new()
        .route("/random", routing::get(random_img))
        .route("/:id", routing::get(find_img_by_id))
        .layer(Extension(img_store));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    log::info!("绑定到: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}