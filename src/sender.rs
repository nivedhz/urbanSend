use crate::discover;
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
    let devices = discover::discover_devices()?;

    if devices.is_empty() {
        println!("No devices found");
        return Ok(());
    }

    println!("Found devices:");
    for device in &devices {
        println!("{}", device);
    }

    // let target_ip = devices[0].ip();

    let mut stream = TcpStream::connect(format!("172.20.10.1:8080"))?;

    println!("Connected to server");

    send_file(stream.peer_addr(), &mut stream, file_path)?;

    Ok(())
}
