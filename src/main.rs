use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Result;
use std::fs;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8001";
const BUFFER_SIZE: usize = 1024;

fn main() {
    let end_point: String = format!("{}:{}", HOST, PORT);
    match TcpListener::bind(end_point) {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream_resp) => {
                        handle_connection(stream_resp);
                    }
                    Err(err) => {
                        eprintln!("Error establishing the connection :: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprint!("Error establishing a TCP listener :: {}", err);
        }
    }
}


fn list_directory(path: &str) -> Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Request Size: {}", bytes_read);
            match fs::read_to_string("./index.html") {
                Ok(response_contents) => {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        response_contents.len(),
                        response_contents
                        );
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(err) => {
                    eprintln!("Error while reading the file :: {}", err);
                    let response = "HTTP/1.1 500 INTERNAL_SERVER_ERROR\r\n\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
        Err(err) => {
            eprintln!("Error Occured while reading the stream :: {}", err);
            let response = "HTTP/1.1 400 BAD_REQUEST\r\n\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
