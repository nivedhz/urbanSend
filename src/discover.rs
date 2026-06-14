use anyhow::Result;
use get_if_addrs::get_if_addrs;
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub fn discover_devices() -> Result<Vec<SocketAddr>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    println!("[*] Bound to {:?}", socket.local_addr()?);
    println!("[*] Sending discovery packets...");

    let mut sent = false;
    if let Ok(interfaces) = get_if_addrs() {
        for iface in interfaces {
            if let get_if_addrs::IfAddr::V4(v4_addr) = iface.addr
                && let Some(broadcast_ip) = v4_addr.broadcast
            {
                let target = SocketAddr::new(std::net::IpAddr::V4(broadcast_ip), 9999);
                if socket.send_to(b"DISCOVER_URBANSEND", target).is_ok() {
                    println!("  ↳ Sent to {} ({})", target, iface.name);
                    sent = true;
                }
            }
        }
    }

    if !sent {
        println!("  ↳ Fallback: Sent to global broadcast 255.255.255.255");
        socket.send_to(b"DISCOVER_URBANSEND", "255.255.255.255:9999")?;
    }

    println!("[*] Waiting for responses...");
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    let mut devices = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("[*] Received {} bytes from {}", size, addr);
                devices.push(addr);
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                    break;
                } else {
                    println!("[!] recv_from error: {}", e);
                    break;
                }
            }
        }
    }

    Ok(devices)
}
