# rust 使用 tonic 简易聊天

## 运行 server 端

- RUST_LOG=info cargo run --example server

## 运行 client 端

- NAME=xxx RUST_LOG=info cargo run --example client

- NAME=yyy RUST_LOG=info cargo run --example client

## 然后就可以在 client 里面发送消息
