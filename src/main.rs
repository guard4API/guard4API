use std::error::Error;

mod server;

#[tokio::main]
async fn main() {
    println!("===== guard4API ====");
    match server::start().await {
        Ok(result) => {
            println!(" Completed ...")
        }
        Err(err) => {
            eprintln!("Error: {:?}", err)
        }
    }
}
