use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;

pub fn run(target: &str) -> Result<Duration, std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    let target: IpAddr = target.parse()?;
    let target_address = SocketAddr::new(target, 7);

    let start_time = std::time::Instant::now();

    socket.send_to(&[0; 1], target_address)?;

    let mut buffer = [0; 1];
    socket.recv_from(&mut buffer)?;

    let duration = start_time.elapsed();

    Ok(duration)
}