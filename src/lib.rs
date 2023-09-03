mod commands;
use commands::{ControlCommand, SetCommand, ReadCommand};

mod socket_utils;
use socket_utils::send_command;

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::collections::HashMap;
use tokio::time::{sleep, timeout, Duration};

//Control Commands
pub async fn execute_control_command(socket: &UdpSocket, drone_addr: &SocketAddr, command: ControlCommand) -> Result<(), Box<dyn std::error::Error>> {
    let command_str = match command {
        ControlCommand::Command => "command".to_string(),
        ControlCommand::Takeoff => "takeoff".to_string(),
        ControlCommand::Land => "land".to_string(),
        ControlCommand::StreamOn => "streamon".to_string(),
        ControlCommand::StreamOff => "streamoff".to_string(),
        ControlCommand::Emergency => "emergency".to_string(),
        ControlCommand::Up(distance) => format!("up {}", distance),
        ControlCommand::Down(distance) => format!("down {}", distance),
        ControlCommand::Left(distance) => format!("left {}", distance),
        ControlCommand::Right(distance) => format!("right {}", distance),
        ControlCommand::Forward(distance) => format!("forward {}", distance),
        ControlCommand::Back(distance) => format!("back {}", distance),
        ControlCommand::RotateClockwise(degrees) => format!("cw {}", degrees),
        ControlCommand::RotateCounterClockwise(degrees) => format!("ccw {}", degrees),
        ControlCommand::Flip(direction) => format!("flip {}", direction),
        ControlCommand::Go(x, y, z, speed) => format!("go {} {} {} {}", x, y, z, speed),
        ControlCommand::Curve(x1, y1, z1, x2, y2, z2, speed) => format!("curve {} {} {} {} {} {} {}", x1, y1, z1, x2, y2, z2, speed),
    };    

    send_command(socket, drone_addr, &command_str).await?;
    Ok(())
}

//Set Commands
pub async fn execute_set_command(socket: &UdpSocket, drone_addr: &SocketAddr, command: SetCommand) -> Result<(), Box<dyn std::error::Error>> {
    let command_str = match command {
        SetCommand::Speed(x) => format!("speed {}", x),
        SetCommand::RC(a, b, c, d) => format!("rc {} {} {} {}", a, b, c, d),
        SetCommand::Wifi(ssid, password) => format!("wifi {} {}", ssid, password),
    };

    send_command(socket, drone_addr, &command_str).await?;
    Ok(())
}

//Read Commands
pub async fn execute_read_command(socket: &UdpSocket, drone_addr: &SocketAddr, command: ReadCommand) -> Result<(), Box<dyn std::error::Error>> {
    let command_str = match command {
        ReadCommand::Speed => "speed?".to_string(),
        ReadCommand::Time => "time?".to_string(),
        ReadCommand::Battery => "battery?".to_string(),
        ReadCommand::Height => "height?".to_string(),
        ReadCommand::Temp => "temp?".to_string(),
        ReadCommand::Attitude => "attitude?".to_string(),
        ReadCommand::Barometer => "baro?".to_string(),
        ReadCommand::Acceleration => "acceleration?".to_string(),
        ReadCommand::TOF => "tof?".to_string(),
        ReadCommand::Wifi => "wifi?".to_string(),
    };

    send_command(socket, drone_addr, &command_str).await?;
    Ok(())
}

//Tello State is on UDP 8890
pub async fn listen_for_state() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 1024];
    let socket = UdpSocket::bind("0.0.0.0:8890").await?;
    loop {
        let (size, _) = socket.recv_from(&mut buffer).await?;
        let received_data = std::str::from_utf8(&buffer[..size])?;
        println!("Received raw state: {}", received_data);

        let mut drone_state: HashMap<String, String> = HashMap::new();
        for pair in received_data.trim().split(';') {
            let key_value: Vec<&str> = pair.split(':').collect();
            if key_value.len() == 2 {
                drone_state.insert(key_value[0].to_string(), key_value[1].to_string());
            }
        }

        println!("Drone state:");
        println!("Pitch: {} degree", drone_state.get("pitch").unwrap_or(&"Unknown".to_string()));
        println!("Roll: {} degree", drone_state.get("roll").unwrap_or(&"Unknown".to_string()));
        println!("Yaw: {} degree", drone_state.get("yaw").unwrap_or(&"Unknown".to_string()));
        println!("Speed X: {}", drone_state.get("vgx").unwrap_or(&"Unknown".to_string()));
        println!("Speed Y: {}", drone_state.get("vgy").unwrap_or(&"Unknown".to_string()));
        println!("Speed Z: {}", drone_state.get("vgz").unwrap_or(&"Unknown".to_string()));
        println!("Lowest Temp: {} °C", drone_state.get("templ").unwrap_or(&"Unknown".to_string()));
        println!("Highest Temp: {} °C", drone_state.get("temph").unwrap_or(&"Unknown".to_string()));
        println!("TOF distance: {} cm", drone_state.get("tof").unwrap_or(&"Unknown".to_string()));
        println!("Height: {} cm", drone_state.get("h").unwrap_or(&"Unknown".to_string()));
        println!("Battery: {} %", drone_state.get("bat").unwrap_or(&"Unknown".to_string()));
        println!("Barometer: {} cm", drone_state.get("baro").unwrap_or(&"Unknown".to_string()));
        println!("Motors on Time: {} s", drone_state.get("time").unwrap_or(&"Unknown".to_string()));
        println!("Acceleration X: {}", drone_state.get("agx").unwrap_or(&"Unknown".to_string()));
        println!("Acceleration Y: {}", drone_state.get("agy").unwrap_or(&"Unknown".to_string()));
        println!("Acceleration Z: {}", drone_state.get("agz").unwrap_or(&"Unknown".to_string()));      
    }
}

//Tello Video Stream is on UDP 111111
pub async fn listen_for_video() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 4096];
    let socket = UdpSocket::bind("0.0.0.0:11111").await?;
    loop {
        let (_size, _) = socket.recv_from(&mut buffer).await?;
        // Process the video data (decode, display, etc.)
    }
}

pub async fn ping(socket: &UdpSocket, drone_addr: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let payload = b"ping";

    loop {
        // Send the payload
        match socket.send_to(payload, drone_addr).await {
            Ok(_) => println!("Ping sent."),
            Err(e) => println!("Failed to send ping: {}", e),
        }

        // Wait for a response with a timeout
        let mut buffer = [0u8; 1024];
        match timeout(Duration::from_secs(2), socket.recv_from(&mut buffer)).await {
            Ok(Ok((amt, _src))) => {
                let text = std::str::from_utf8(&buffer[0..amt])?;
                println!("Received response: {}", text);
            },
            Ok(Err(e)) => println!("Failed to receive response: {}", e),
            Err(_) => println!("Response timed out"),
        }
        
        // Wait before sending the next ping
        sleep(Duration::from_secs(1)).await;
    }
}
