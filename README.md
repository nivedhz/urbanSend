# urbanSend

A lightweight file transfer application written in Rust.

urbanSend is an experimental project focused on understanding how modern peer-to-peer file sharing applications work under the hood. The long-term goal is to create a fast, cross-platform file transfer tool inspired by solutions like LocalSend, while learning networking, systems programming, and Rust along the way.

> ## 🚧 Heavy Development Notice
>
> urbanSend is heavily under development.
>
> Features, networking behavior, and internal architecture may change at any time. Expect bugs, breaking changes, unfinished functionality, and protocol modifications as the project evolves.
>
> The primary goal right now is learning, experimentation, and building the foundations of a future cross-platform file transfer application.

## Current Features

- Send files over TCP
- Receive files over TCP
- Supports large files (tested with files over 1 GB)
- IPv6 support
- No external servers
- Pure Rust implementation

## Current Status

The project can:

1. Start a receiver that listens for incoming connections.
2. Connect a sender to the receiver.
3. Transfer a file.
4. Save the received file on the destination machine.

## Usage

### Start the Receiver

Open a terminal and run:

```bash
cargo run recieve
```

The receiver will start listening for incoming file transfers.

### Send a File

Open another terminal and run:

```bash
cargo run send /path/to/file
```

Example:

```bash
cargo run send /home/user/Videos/video.mkv
```

The sender will:

1. Connect to the receiver.
2. Transfer the file.
3. Disconnect automatically after the transfer completes.

## Example Output

Receiver:

```code
Server listening on 8080
Discovery service listening on 9999
Sending response back directly to 198.51.100.50
Sent 14 bytes
Connection established from 198.51.100.50
Receiving file: video.mkv (1249771520 bytes)
Received file: video.mkv -> Saved to: "/home/user/Downloads/urbanSend/video.mkv"
198.51.100.50disconnected
```

Sender:

```code
Bound to 0.0.0.0:57354
Waiting for responses...
Received 14 bytes from 198.51.100.50
Found devices:
198.51.100.50
Connected to server at 198.51.100.50
Sent "movie.mkv" of 1249771520 bytes to 198.51.100.50:8080
```

## Roadmap

### Networking

- [ ] Automatic device discovery
- [ ] Multiple device support
- [ ] Transfer progress reporting
- [ ] Transfer speed metrics
- [ ] Resume interrupted transfers

* [ ] Multiple device support
* [ ] Transfer progress reporting
* [ ] Transfer speed metrics
* [ ] Resume interrupted transfers

### Reliability

- [ ] File integrity verification
- [ ] Better error handling
- [ ] Transfer confirmations

### Security

- [ ] Device authentication
- [ ] End-to-end encryption

### Platform Support

- [ ] Desktop application
- [ ] Android application
- [ ] iOS application
- [ ] Cross-platform GUI

## Why?

This project exists primarily as a learning journey.

Rather than treating networking as a black box, urbanSend is being built from the ground up to explore:

- TCP/IP networking
- File transfer protocols
- Service discovery
- Concurrency
- Systems programming
- Rust

Every feature is implemented incrementally with the goal of understanding the underlying technology rather than simply using it.

## License

MIT
