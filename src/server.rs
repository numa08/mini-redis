use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::store::Store;
use crate::command::Command;

pub async fn run(addr: &str, store: Store) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New connection from: {}", addr);

        let store = store.clone();
        tokio::spawn(async move {
            handle_connection(socket, store).await;
        });
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream, store: Store) {
    let mut buf = [0u8; 512];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("Connection closed");
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buf[..n]);
                let received = received.trim();
                println!("Received: {:?}", received);

                let command = Command::parse(received);
                let response = command.execute(&store);

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    println!("Error writing: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("Error reading: {}", e);
                break;
            }
        }
    }
}
