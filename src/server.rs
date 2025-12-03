use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::command::Command;
use crate::store::Store;

pub async fn run(addr: &str, store: Store) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        let store = store.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, store).await {
                println!("Connection error: {}", e);
            };
        });
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream, store: Store) -> io::Result<()> {
    let mut buf = [0u8; 512];

    loop {
        match socket.read(&mut buf).await? {
            0 => {
                println!("Connection closed");
                break;
            }
            n => {
                let received = String::from_utf8_lossy(&buf[..n]);
                let received = received.trim();
                println!("Received: {:?}", received);

                let command = Command::parse(received);
                let response = command.execute(&store);

                socket.write_all(response.as_bytes()).await?;
            }
        }
    }

    Ok(())
}
