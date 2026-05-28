use std::io::{self, Write};
use std::net::TcpStream;

fn send_data() -> std::io::Result<()> {
    let port = 8080;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;

    println!("Connected to server");

    loop {
        print!("Enter a message: ");
        io::stdout().flush()?;

        let mut message = String::new();

        io::stdin().read_line(&mut message)?;

        let trimmed_message = message.trim();

        if trimmed_message == "exit" {
            println!("Disconnecting...");
            break;
        }

        stream.write_all(trimmed_message.as_bytes())?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    send_data()
}
