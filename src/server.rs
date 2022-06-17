use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub async fn start() {
    let listen_addr = "127.0.0.1:8080".to_string();
    println!("Server listing on: {}", listen_addr);
    let listener = TcpListener::bind(listen_addr).await.unwrap();
    loop {
        let (socket, address) = listener.accept().await.unwrap();
        handle_connection(socket, address).await;
    }
}

async fn handle_connection(mut socket: TcpStream, _address: SocketAddr) {
    let http_header_first = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-length: ";
    let http_header_last = "\r\nServer: guard-api/0.1\r\nAccept-Ranges: bytes\r\n\r\n";

    tokio::spawn(async move {
        let (read, mut writer) = socket.split();
        let mut reader = BufReader::new(read);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            if bytes_read == 0 {
                // sending same data
                let data = String::from("<b>It works!</b>");
                let size: usize = data.len();
                let mut response = String::from(http_header_first);
                response.push_str(&size.to_string());
                response.push_str(http_header_last);
                writer.write_all(response.as_bytes()).await.unwrap();
                line.clear();
                break;
            }
            println!("Message Received :: {}", line);
        }
    });
}
