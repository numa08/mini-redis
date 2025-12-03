mod command;
mod server;
mod store;

use std::time::Duration;

use store::Store;

#[tokio::main]
async fn main() {
    let store = Store::new();

    let cleanup_store = store.clone();
    // バックグラウンドでクリーンアップ
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            cleanup_store.cleanup_expired();
        }
    });
    if let Err(e) = server::run("127.0.0.1:6379", store).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    };
}
