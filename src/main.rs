mod server;

#[tokio::main]
async fn main() {
    server::run("127.0.0.1:6379").await;
}
