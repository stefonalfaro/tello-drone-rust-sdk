use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tello_sdk::ping;
use tello_sdk::listen_for_state;

fn main() {
    println!("Initializing runtime and other setup.");

    let task = async {
        let drone_addr: SocketAddr = "192.168.10.1:8889".parse().unwrap();
        let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        let mut ping_successful = false;

        // Ping loop
        while !ping_successful {
            match ping(&socket, &drone_addr).await {
                Ok(_) => {
                    println!("Ping successful. Moving to next stage.");
                    ping_successful = true;
                }
                Err(e) => {
                    println!("Ping failed: {}. Retrying...", e);
                }
            }
        }

        // State listening loop
        match listen_for_state().await {
            Ok(_) => {
                println!("State listening successful.");
            }
            Err(e) => {
                println!("State listening failed: {}", e);
            }
        }
    };

    tokio::runtime::Runtime::new().unwrap().block_on(task);
}