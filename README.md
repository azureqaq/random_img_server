# random_img_server

简单的局域网随机图片 API 服务器

以 API 的方式获取(随机)主机多个目录下的 `jpg` 图片

## 如何使用

### 配置文件
- 按照 [示例](./config_template.toml) 填写
- 可通过命令行参数指定不同的配置文件 *默认：./config.toml*

### 执行
- `img <CONFIG_PATH>` 不使用参数则为默认位置: `./config.toml`

### API 接口
- 获取随机图片: GET http://host:port/random
- 获取指定的图片: GET http://host:port/ID/pic.jpg *替换 ID*

## 注意
- 不要在公网环境中使用
