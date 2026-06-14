use crate::discover;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::path::Path;
use std::str::FromStr;

fn send_file(
    addr: Result<SocketAddr, std::io::Error>,
    stream: &mut TcpStream,
    file_path: &str,
) -> Result<()> {
    let path = Path::new(file_path.trim());
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    stream.write_all(b"FILE")?;
    let file_name_len = file_name.len() as u64;
    stream.write_all(&file_name_len.to_be_bytes())?;
    stream.write_all(file_name.as_bytes())?;
    stream.write_all(&file_size.to_be_bytes())?;

    const MEGABYTES: usize = 1;
    let mut buffer = [0; MEGABYTES * 1024 * 1024];
    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.yellow/orange}] {bytes}/{total_bytes} ({eta})").unwrap().progress_chars("|>-"));

    loop {
        let bytes_read = file.read(&mut buffer)?;
        progress_bar.inc(u64::try_from(bytes_read).unwrap());

        if bytes_read == 0 {
            progress_bar.finish();
            break;
        }
        stream.write_all(&buffer[..bytes_read])?;
    }

    println!(
        "[*] Sent {:?} of {}MB to {:?}",
        file_name,
        file_size / 1024 / 1024,
        addr.unwrap()
    );

    Ok(())
}

pub fn send_data(file_path: &str) -> Result<()> {
    let devices = discover::discover_devices()?;
    let target_ip: IpAddr;

    if devices.is_empty() {
        println!("\n[!] No devices found automatically via UDP Broadcast.");
        println!("[*] Android or network topology may be blocking discovery traffic.");

        loop {
            print!("[*] Please enter the Receiver's IP address manually: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let trimmed_input = input.trim();

            match IpAddr::from_str(trimmed_input) {
                Ok(ip) => {
                    target_ip = ip;
                    break;
                }
                Err(_) => {
                    println!("[!] Invalid IP format. Please try again (e.g., 192.168.43.50).");
                }
            }
        }
    } else {
        println!("[*] Found devices automatically:");
        let mut device_count = 0;
        for device in &devices {
            device_count += 1;
            println!("{device_count}) {device}");
        }
        println!();
        target_ip = devices[0].ip();
    }

    println!(
        "[*] Attempting connection to server at {}:8080...",
        target_ip
    );
    let mut stream = TcpStream::connect(format!("{}:8080", target_ip))?;
    println!("[*] Connected to server successfully!");

    send_file(stream.peer_addr(), &mut stream, file_path)?;

    Ok(())
}
