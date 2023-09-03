# Tello Rust SDK

## Overview

The Tello Rust SDK provides a high-level API for interacting with the DJI Tello drone in Rust. It handles connecting to the drone over UDP, sending commands, and receiving state information and video streams.

The SDK is composed of several modules:

- `commands` - Defines enums for the various command types.
- `lib` - The main library containing high level control functions.
- `socket_utils` - Lower level UDP socket helpers.

## Usage

First, create a `UdpSocket` and get the drone's IP address:

```rust
let socket = UdpSocket::bind("0.0.0.0:0").await?;
let drone_addr: SocketAddr = "192.168.10.1:8889".parse()?;
``` 

### Sending Commands

To send control commands, call `execute_control_command` with the command enum:

```rust
use tello_sdk::execute_control_command;

let takeoff_command = ControlCommand::Takeoff;
execute_control_command(&socket, &drone_addr, takeoff_command).await?;
```

Similar functions exist for `SetCommand` and `ReadCommand`.

### Getting State

To listen for state information from the drone, call the `listen_for_state` function:

```rust 
use tello_sdk::listen_for_state;

listen_for_state().await?;
```

This will continuously print out decoded state messages.

### Video Streaming

To receive the video stream, call `listen_for_video`:

```rust
use tello_sdk::listen_for_video; 

listen_for_video().await?;
```

This will receive raw video packets that still need to be decoded.

### Ping

The `ping` function can be used to test connectivity with the drone:

```rust
use tello_sdk::ping;

ping(&socket, &drone_addr).await?; 
```

It will send pings and wait for responses.

## Examples

See the `examples` folder for a sample application that pings the drone and listens for state messages.

Let me know if any part of the documentation needs more explanation! I tried to cover the key points to get started using the SDK.