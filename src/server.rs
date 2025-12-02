use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream) {
    let mut buf = [0u8; 512];

    loop {
        match socket.read(&mut buf).await {
            Ok(n) if n == 0 => {
                println!("Connection closed");
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buf[..n]);
                println!("Received: {:?}", received);
                if let Err(e) = socket.write_all(&buf[..n]).await {
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
