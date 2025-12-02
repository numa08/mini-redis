mod server;
mod store;

use store::Store;

#[tokio::main]
async fn main() {
    let store = Store::new();
    server::run("127.0.0.1:6379", store).await;
}
