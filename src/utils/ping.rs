pub fn perform_ping(target: &str) -> Result<Duration, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("Socket bind error: {}", e))?;

    // Set a timeout for the receive operation
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("Socket set_read_timeout error: {}", e))?;

    let ip_addr = resolve_ip(target)?;

    let packet = build_icmp_echo_request();

    let start_time = Instant::now();

    // Send ICMP Echo Request
    socket
        .send_to(&packet, SocketAddr::new(ip_addr, 0))
        .map_err(|e| format!("Socket send_to error: {}", e))?;

    // Receive ICMP Echo Reply
    let mut reply_packet = [0u8; 28]; // Adjust the size accordingly
    socket
        .recv_from(&mut reply_packet)
        .map_err(|e| format!("Socket recv_from error: {}", e))?;

    let end_time = Instant::now();

    // Calculate round trip time
    let duration = end_time - start_time;

    Ok(duration)
}

pub fn resolve_ip(target: &str) -> Result<Ipv4Addr, String> {
    target
        .parse()
        .map_err(|e| format!("Failed to parse IP address: {}", e))
}

pub fn build_icmp_echo_request() -> Vec<u8> {
    let mut packet = vec![0u8; ICMP_HEADER_SIZE];

    // ICMP Echo Request
    packet[0] = ICMP_ECHO_REQUEST;

