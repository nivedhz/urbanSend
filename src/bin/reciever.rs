use std::fs::{self, File};
use std::io::{BufWriter, Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("Connection established from {:?}", stream.peer_addr()?);

    loop {
        let mut packet_type = [0; 4];
        if stream.read_exact(&mut packet_type).is_err() {
            println!("{} disconnected", stream.peer_addr()?);
            break;
        }

        match &packet_type {
            b"FILE" => {
                let mut name_len_buf = [0; 8];
                stream.read_exact(&mut name_len_buf)?;
                let name_len = u64::from_be_bytes(name_len_buf);

                let mut name_buf = vec![0; name_len as usize];
                stream.read_exact(&mut name_buf)?;
                let filename = String::from_utf8_lossy(&name_buf);

                let mut file_size_buf = [0; 8];
                stream.read_exact(&mut file_size_buf)?;
                let file_size = u64::from_be_bytes(file_size_buf);

                println!("Receiving file: {} ({} bytes)", filename, file_size);

                let folder_path = "/data/data/com.termux/files/home/storage/downloads/urbanSend/";
                fs::create_dir_all(folder_path)?;
                let save_path = format!("{}{}", folder_path, filename);

                let file = File::create(save_path)?;
                let mut writer = BufWriter::new(file);
                let mut remaining = file_size;
                let mut buffer = [0; 1048576];

                while remaining > 0 {
                    let bytes_to_read = std::cmp::min(buffer.len() as u64, remaining) as usize;
                    let bytes_read = stream.read(&mut buffer[..bytes_to_read])?;
                    if bytes_read == 0 {
                        break;
                    }
                    writer.write_all(&buffer[..bytes_read])?;
                    remaining -= bytes_read as u64;
                }

                writer.flush()?;
                println!("Received file: {}", filename);
            }
            other => {
                println!("Packet type: {:?}", String::from_utf8_lossy(other));
            }
        }
    }

    Ok(())
}

fn recieve_data() -> Result<()> {
    let port = 8080;

    let listener = TcpListener::bind(format!("[::]:{}", port))?;

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
