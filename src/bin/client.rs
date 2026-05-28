use std::io::{self, Result, prelude::*};
use std::net::TcpStream;

fn send_data() -> std::io::Result<()> {
    let port: i32 = 8080;

    loop {
        let mut stream: TcpStream = TcpStream::connect(format!("127.0.0.1:{:?}", port))?;
        print!("Enter a message: ");
        io::stdout().flush().expect("Failed to flush out message");

        let mut message: String = String::new();

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let trimmed_message = message.trim();
        if trimmed_message == "exit" {
            break;
        }

        stream.write_all(message.as_bytes())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    send_data()
}
