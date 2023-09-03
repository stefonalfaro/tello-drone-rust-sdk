// Enum to represent the possible responses for any command
#[derive(Debug)]
pub enum CommandResponse {
    Ok,
    Error(String),  // An informative result code can be encapsulated here
}

// Enums to represent each command type
#[derive(Debug)]
pub enum ControlCommand {
    Command,
    Takeoff,
    Land,
    StreamOn,
    StreamOff,
    Emergency,
    Up(u32),        // distance x in cm, range 20-500
    Down(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
    Back(u32),
    RotateClockwise(u32),  // x: 1-3600
    RotateCounterClockwise(u32),
    Flip(char),  // 'l': left, 'r': right, 'f': forward, 'b': back
    Go(u32, u32, u32, u32),  // x, y, z, speed
    Curve(u32, u32, u32, u32, u32, u32, u32),  // x1, y1, z1, x2, y2, z2, speed
}

#[derive(Debug)]
pub enum SetCommand {
    Speed(u32),  // x: 10-100
    RC(i32, i32, i32, i32),  // a, b, c, d
    Wifi(String, String),  // ssid, password
}

#[derive(Debug)]
pub enum ReadCommand {
    Speed,
    Time,
    Battery,
    Height,
    Temp,
    Attitude,
    Barometer,
    Acceleration,
    TOF,
    Wifi,
}