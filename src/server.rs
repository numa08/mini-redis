use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::store::Store;

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

                let response = handle_command(received, &store);
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

fn handle_command(input: &str, store: &Store) -> String {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        ["PING"] => "+PONG\r\n".to_string(),
        ["SET", key, value] => {
            store.set(key.to_string(), value.to_string());
            "+OK\r\n".to_string()
        }
        ["GET", key] => match store.get(key) {
            Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
            None => "$-1\r\n".to_string(),
        },
        _ => "-Err unknown command\r\n".to_string(),
    }
}
