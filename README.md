# Accmo - Accelerometer Motion Visualizer

A real-time accelerometer data visualization tool that displays your device's motion in a terminal UI. Built with Rust, it uses a web interface to capture device motion and displays the data in a beautiful TUI with graphs and gauges.

## Features

- Real-time accelerometer data capture from mobile devices
- Terminal-based UI with live graphs and gauges
- Historical data visualization
- Color-coded axis representation



## Installation

1. Clone the repository:


2. Generate SSL certificates (required for HTTPS and iOS accelerometer access):
```bash
mkdir certs
cd certs
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/CN=localhost"
cd ..
```

3. Build and run:
```bash
cargo run
```

## Usage

1. Start the server:
```bash
cargo run
```

2. On your mobile device:
   - Connect to the same network as your computer
   - Open a browser and navigate to `https://<your-computer-ip>:3000`
   - Accept the self-signed certificate warning
   - Click "Request Motion Permission" button
   - Move your device around to see the visualization

3. In the terminal:
   - Watch the real-time graphs and gauges update
   - Press 'q' to quit the application

## Architecture

- `src/main.rs`: Main server implementation with HTTPS and WebSocket support
- `src/tui.rs`: Terminal UI implementation using ratatui
- `static/`: Web interface files
  - `index.html`: Main webpage
  - `script.js`: Client-side JavaScript for accelerometer access

## Controls

- 'q': Quit the application
- The web interface automatically connects and starts sending data once permission is granted
