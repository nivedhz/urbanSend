use std::fs::File;
use std::io::{self, Read, Result, Write};
use std::net::TcpStream;
use std::path::Path;

fn send_file(addr: &str, stream: &mut TcpStream) -> Result<()> {
    loop {
        print!("Enter the file path: ");
        io::stdout().flush()?;
        let mut file_path = String::new();
        io::stdin().read_line(&mut file_path)?;
        if file_path == "exit" {
            break;
        }

        let path = Path::new(file_path.trim());
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // packet type
        stream.write_all(b"FILE")?;
        // filename length
        let file_name_len = file_name.len() as u64;
        stream.write_all(&file_name_len.to_be_bytes())?;
        // filename
        stream.write_all(file_name.as_bytes())?;
        // file size
        stream.write_all(&file_size.to_be_bytes())?;
        // file data
        let mut buffer = [0; 1048576];

        loop {
            let bytes_read = file.read(&mut buffer)?;

            if file_path.trim() == "exit" {
                break;
            }
            if bytes_read == 0 {
                break;
            }
            stream.write_all(&buffer[..bytes_read])?;
        }

        println!("Sent {:?} of {} bytes to {}", file_name, file_size, addr);
    }

    Ok(())
}

fn send_data() -> std::io::Result<()> {
    let port = 8080;
    let mut stream = TcpStream::connect("[2409:40f3:d:5555:e021:f3ff:febc:dd83]:8080")?;

    println!("Connected to server");

    send_file(format!("127.0.0.1:{}", port).as_str(), &mut stream)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    send_data()
}
