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

    fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    fn generate_request_header(&self) -> String {
        let mut header_info = String::new();
        header_info.push_str(self.method.as_str());
        header_info.push_str(" ");

        header_info.push_str(self.uri.as_str());
        header_info.push_str(" ");

        header_info.push_str(self.version.as_str());
        header_info.push_str("\r\n");

        for (key, value) in &self.headers {
            header_info.push_str(format!("{}: {}", key, value).as_str());
            header_info.push_str("\r\n");
        }
        return header_info;
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
                let mut request_header = RequestMetaInfo::from(&request_headers)?;
                request_header.add_header(String::from("X-Forwarded"), String::from("guard4API"));
                println!(" Headers: {:?}", request_header);
                server_writer
                    .write(request_header.generate_request_header().as_bytes())
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
