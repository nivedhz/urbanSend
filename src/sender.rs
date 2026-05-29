use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;

fn send_file(
    addr: Result<SocketAddr, std::io::Error>,
    stream: &mut TcpStream,
    file_path: String,
) -> Result<()> {
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

        if bytes_read == 0 {
            break;
        }
        stream.write_all(&buffer[..bytes_read])?;
    }

    println!(
        "Sent {:?} of {} bytes to {:?}",
        file_name,
        file_size,
        addr.unwrap()
    );

    Ok(())
}

pub fn send_data(file_path: String) -> Result<()> {
    // let port = 8080;
    let mut stream = TcpStream::connect("[2409:40f3:d:5555:e021:f3ff:febc:dd83]:8080")?;
    // let mut stream = TcpStream::connect(format!("[::1]:{}", port))?;

    println!("Connected to server");

    send_file(stream.peer_addr(), &mut stream, file_path)?;
    Ok(())
}
