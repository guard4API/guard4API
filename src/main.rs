mod server;

#[tokio::main]
async fn main() {
    println!("===== guard4API ====");
    server::start().await;
}
