use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub async fn send_command(
    socket: &UdpSocket,
    drone_addr: &SocketAddr,
    command: &str
) -> Result<(), Box<dyn std::error::Error>> {
    socket.send_to(command.as_bytes(), drone_addr).await?;
    Ok(())
}
