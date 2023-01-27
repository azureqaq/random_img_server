# random_img_server

简单的局域网随机图片 API 服务器

以 API 的方式获取(随机)主机多个目录下的 `jpg` 图片

## 如何使用

### 配置文件
- 按照 [示例](./config_template.toml) 创建并填写 `config.toml`：

  > ```toml
  > # ip 地址
  > ip = [127, 0, 0, 1]
  > 
  > # 绑定的端口
  > port = 7878
  > 
  > # 图片所处文件夹
  > dirs = [
  >     "./imgs",
  >     "./imgs_1"
  > ]
  > ```

- 可通过命令行参数指定不同的配置文件 *默认：./config.toml*

- **注意:** 如果即不指定配置文件，也没有可用的默认配置文件，则会使用去掉 `./imgs_1` 的上述示例配置

### 执行
- `img <CONFIG_PATH>` 不使用参数则为默认位置: `./config.toml`

### API 接口
- 获取随机图片: *GET http://host:port/random*
- 获取指定的图片: *GET http://host:port/ID/pic.jpg*

### 调用

- Python

  > ```python
  > import requests
  > 
  > if __name__ = '__main__':
  >     # 发起请求
  >     result = requests.get('http://host:port/random')
  >     # 检查状态码
  >     if result.ok:
  >         # 保存到本地文件
  >         with open('img.jpg', 'wb') as fr:
  >             fr.write(result.content)
  >     else:
  >         print('获取随即图片失败')  
  > ```

- Rust

  > ```rust
  > use reqwest;
  > use tokio;
  > 
  > #[tokio::main]
  > async fn main() -> Result<(), Box<dyn std::error::Error>> {
  >     // 指定获取第 0 个图片
  >     let resp = reqwest::get("http://host:port/0/pic.jpg").await?;
  >     if resp.status().is_success() {
  >         println!("获取成功");
  >         // 其他操作 ...
  >         Ok(())
  >     } else {
  >         Err("获取失败")
  >     }
  > }
  > ```

## 注意

- 不要在公网环境中使用
- 仅支持 `jpg` 格式图片
- 图片数量限制: `<= usize::MAX = 4_294_967_295u32 or 18_446_744_073_709_551_615u64`
- 图片库更改时，请重启程序
- 通过 ID 方式获取时，可通过状态码判断是否为正常图片

