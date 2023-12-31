use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::fs;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8001";
const BUFFER_SIZE: usize = 1024;

#[tokio::main]
async fn main() {
    let end_point: String = format!("{}:{}", HOST, PORT);
    match TcpListener::bind(&end_point).await {
        Ok(listener) => {
            println!("Listening on {}", end_point);
            while let Ok((socket, _)) = listener.accept().await {
                tokio::spawn(handle_connection(socket));
            }
        }
        Err(err) => {
            eprintln!("Error binding to address: {}", err);
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer).await?;

    let file_content = fs::read_to_string("./index.html").await?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        file_content.len(),
        file_content
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}