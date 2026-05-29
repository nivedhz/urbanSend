use anyhow::Result;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub fn discover_devices() -> Result<Vec<SocketAddr>> {
    let socket = UdpSocket::bind("0.0.0.0:9998")?;

    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    let _ = socket.send_to(b"DISCOVER_URBANSEND", "255.255.255.255:9999")?;
    let _ = socket.send_to(b"DISCOVER_URBANSEND", "127.0.0.1:9999")?;

    let mut devices = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((_, addr)) => {
                devices.push(addr);
            }
            Err(_) => break,
        }
    }

    Ok(devices)
}
