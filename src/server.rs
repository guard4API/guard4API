use futures::FutureExt;
use std::collections::HashMap;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct RequestMetaInfo {
    method: String,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
}

impl RequestMetaInfo {
    fn new() -> RequestMetaInfo {
        RequestMetaInfo {
            method: "".to_string(),
            uri: "".to_string(),
            version: "".to_string(),
            headers: Default::default(),
        }
    }
    //parse request info from string
    fn from(info: &String) -> Result<RequestMetaInfo, Box<dyn Error>> {
        let mut request_info = RequestMetaInfo::new();
        request_info.parse_method(&info);
        request_info.parse_uri(&info);
        request_info.parse_version(&info);
        request_info.parse_headers(&info);
        Ok(request_info)
    }

    fn parse_method(&mut self, info: &String) {
        if !info.trim().is_empty() {
            let mut itr = info.trim().split_whitespace();
            self.method = itr.next().unwrap().to_string();
        }
    }
    fn parse_uri(&mut self, info: &String) {
        if !info.trim().is_empty() {
            let mut itr = info.trim().split_whitespace();
            itr.next(); // just move one step from method
            self.uri = itr.next().unwrap().to_string();
        }
    }
    fn parse_version(&mut self, info: &String) {
        if !info.trim().is_empty() {
            let mut itr = info.trim().split_whitespace();
            itr.next(); // just move one step from method and uri
            itr.next();
            self.version = itr.next().unwrap().to_string();
        }
    }

    fn parse_headers(&mut self, info: &String) {
        if !info.trim().is_empty() {
            let mut lines = info.trim().lines();
            lines.next(); // just skip first line
            for full_header in lines {
                let mut header_part: Vec<&str> = full_header.split(":").collect();
                if !header_part.is_empty() {
                    self.headers.insert(
                        header_part[0].to_string(),
                        header_part[1].trim_start().to_string(),
                    );
                }
            }
        }
    }
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    let listen_addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let handle_connection = handle_connection(inbound).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(handle_connection);
    }

    Ok(())
}

async fn handle_connection(mut incoming_stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let server_addr = String::from("127.0.0.1:8888");
    let mut server_stream = TcpStream::connect(server_addr).await?;

    let (incoming_reader, mut incoming_writer) = incoming_stream.split();
    let (_server_reader, mut server_writer) = server_stream.split();

    let mut reader = BufReader::new(incoming_reader);
    let mut line = String::new();
    let mut request_headers = String::new();
    // read line by line
    while let Ok(size_read) = reader.read_line(&mut line).await {
        if size_read > 0 {
            if line == "\r\n" {
                let request_header = RequestMetaInfo::from(&request_headers)?;
                println!(" Headers: {:?}", request_header);
                server_writer
                    .write(request_headers.as_bytes())
                    .await
                    .unwrap();
            } else {
                request_headers.push_str(&line);
            }
            line.clear();
            //thread::sleep(Duration::from_millis(500));
        } else {
            match server_writer.shutdown().await {
                Ok(_) => {
                    println!("closing server writer...");
                }
                Err(_) => {}
            };
            match incoming_writer.shutdown().await {
                Ok(_) => {
                    println!("closing incoming writer...")
                }
                Err(_) => {}
            };
        }
    }

    /*
        let client_to_server = async {
            io::copy(&mut incoming_reader, &mut server_writer).await?;
            server_writer.shutdown().await
        };

    let server_to_client = async {
        io::copy(&mut server_reader, &mut incoming_writer).await?;
        incoming_writer.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?; */
    Ok(())
}

/*
async fn handle_connection(mut socket: TcpStream, _address: SocketAddr) {
    // let http_header_first = "HTTP/1.1 200 OK\r\n\r\n";
    //let http_header_last = "\r\nServer: guard-api/0.1\r\nAccept-Ranges: bytes\r\n\r\n";

    tokio::spawn(async move {
        println!(" New Client Connected ...");
        let (read, mut writer) = socket.split();
        let mut reader = BufReader::new(read);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            println!(" Byte read {}", bytes_read);
            if bytes_read == 0 {
                // sending same data
                //let data = String::from("<b>It works!</b>");
                //let size: usize = data.len();
                //  let mut response = String::from(http_header_first);
                // response.push_str(&size.to_string());
                // response.push_str(http_header_last);
                // writer.write_all(response.as_bytes()).await.unwrap();
                //writer.flush().await.unwrap();
                // line.clear();
                break;
            }
            println!("Message Received :: {}", line);
            //forward request to the target server
            forward_to_server(line.as_bytes()).await;
        }
    });
}

async fn forward_to_server(data: &[u8]) {
    let server_addr = String::from("127.0.0.1:8888");
    println!("trying to connect with server : {:?}", server_addr);
    match TcpStream::connect(server_addr).await {
        Err(e) => eprintln!(" unable to connect with server. {:?} ", e),

        Ok(mut client) => {
            println!("connected with server : {}", client.local_addr().unwrap());
            let (read, mut writer) = client.split();
            let mut reader = BufReader::new(read);


            writer.write(data).await.unwrap();
            writer.flush();
            /*
            // 1.send data to the server
            let client_to_server = async{
                writer.write(data).await?;
            };
            // 2. read response from server

            loop {
                let mut line = String::new();
                let read_result = reader.read_line(&mut line).await;
                match read_result {
                    Err(e) => println!("Error while reading ...{:?}", e),
                    Ok(bytes_read) => {
                        println!("Byte read {}", bytes_read);
                        if bytes_read == 0 {
                            break;
                        }
                        println!("Message Received :: {}", line);
                    }
                }
            }
            // 3. send to the proxy server */
        }
    }
}
*/
