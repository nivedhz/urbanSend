use anyhow::Result;
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub fn discover_devices() -> Result<Vec<SocketAddr>> {
    // Bind to port 0 to let the OS pick a random ephemeral port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    println!("Bound to {:?}", socket.local_addr()?);

    socket.set_broadcast(true)?;
    println!("Waiting for responses...");

    // Set a 2-second timeout for discovery
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    // Send to the global broadcast address instead of a hardcoded IP
    socket.send_to(b"DISCOVER_URBANSEND", "255.255.255.255:9999")?;

    let mut devices = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("Received {} bytes from {}", size, addr);
                devices.push(addr);
            }
            Err(e) => {
                // Gracefully exit the loop when the 2-second timeout hits
                if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                    break;
                } else {
                    println!("recv_from error: {}", e);
                    break;
                }
            }
        }
    }

    Ok(devices)
}

