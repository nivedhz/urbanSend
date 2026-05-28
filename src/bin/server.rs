use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("Connection established from {:?}", stream.peer_addr()?);

    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string();

        println!("Received Data: {:?}", message);
    }

    Ok(())
}

fn recieve_data() -> Result<()> {
    let port = 8080;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    println!("Server listening on {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }

            Err(e) => {
                eprintln!("Connection failed: {:?}", e);
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    recieve_data()
}
