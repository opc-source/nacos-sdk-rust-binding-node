# nacos-sdk-rust-binding-node
nacos-sdk-rust binding for NodeJs with napi.

Tip: nacos-sdk-nodejs 仓库暂未提供 2.x gRPC 交互模式，为了能升级它，故而通过 node addon 方式调用 nacos-sdk-rust 

# Usage
**使用样例请看仓库内的 example 目录，完整 api 请看 index.d.ts**

环境变量 `NACOS_CLIENT_LOGGER_LEVEL=INFO` 可设置日志打印级别，默认 INFO 
- 客户端日志请在目录 $HOME/logs/nacos/ 查看

npm 包 -> https://www.npmjs.com/package/nacos-sdk-rust-binding-node

# License
[Apache License Version 2.0](LICENSE)

# Acknowledgement
- binding for NodeJs with napi by [napi-rs](https://github.com/napi-rs/napi-rs.git)
- binding the [nacos-sdk-rust](https://github.com/nacos-group/nacos-sdk-rust.git)
