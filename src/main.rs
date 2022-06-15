use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use std::str;

#[tokio::main]
async fn main() {
    println!("===== guard4API ====");

    let listen_addr = "127.0.0.1:8080".to_string();
    println!("Listening on: {}", listen_addr);
    let listener = TcpListener::bind(listen_addr).await.unwrap();
    loop {
        let (mut socket, address) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (read, mut writer) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    // sending same data
                    let mut response = "HTTP/1.1 200 OK\r\n".to_string();
                    response.push("ContentType: text/html\r\n".to_string().parse().unwrap());
                    response.push("Connection: close\r\n".to_string().parse().unwrap());
                    response.push("\r\n".to_string().parse().unwrap());
                    response.push("<b>It works!</b>".to_string().parse().unwrap());
                    response.push("\r\n\r\n".to_string().parse().unwrap());
                    writer.write_all(response.as_bytes()).await.unwrap();
                    line.clear();
                    break;
                }
                println!("Message Received :: {}", line);
            }
        });
    }
}
