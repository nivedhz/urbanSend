use std::io::{Read, Result};
use std::net::TcpListener;
use std::net::TcpStream;

fn recieve_data() -> std::io::Result<()> {
    let port: i32 = 8080;
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{:?}", port))?;
    println!("Server listening on {:?}", port);

    for stream in listener.incoming() {
        let mut stream: TcpStream = stream?;

        let mut buffer = [0; 512];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("{:?} Disconnected", port);
                } else {
                    println!(
                        "Recieved Data: {:?}",
                        String::from_utf8_lossy(&buffer[..bytes_read]).trim()
                    );
                }
            }
            Err(e) => eprint!("Error Occured: {:?}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    recieve_data()
}
