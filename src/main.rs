use std::error::Error;

mod protocol;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_address = "127.0.0.1:25565";

    println!("Starting server on {}...", server_address);
    server::run_server(server_address).await
}
