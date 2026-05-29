use anyhow::Result;
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration; // Import ErrorKind to check for timeouts

pub fn discover_devices() -> Result<Vec<SocketAddr>> {
    // let socket = UdpSocket::bind("0.0.0.0:9998")?;
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    println!("Bound to {:?}", socket.local_addr()?);

    socket.set_broadcast(true)?;
    println!("Waiting for responses...");
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    // CHANGE: Send to the global broadcast address instead of a hardcoded IP
    socket.send_to(b"DISCOVER_URBANSEND", "255.255.255.255:9999")?;

    let mut devices = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("Received {} bytes from {}", size, addr);
                println!("Received response from {}", addr);
                devices.push(addr);
            }
            Err(e) => {
                // CHANGE: Gracefully handle the timeout (OS Error 11 / WouldBlock)
                if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                    break; // Expected timeout, exit the loop cleanly
                } else {
                    println!("recv_from error: {}", e);
                    break;
                }
            }
        }
    }

    Ok(devices)
}
