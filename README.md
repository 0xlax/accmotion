# 📱  Accelerometer Motion Visualizer

<div align="center">

![GitHub](https://img.shields.io/github/license/0xlax/accmo)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-4D4D4D?style=for-the-badge&logo=windows%20terminal&logoColor=white)

A real-time accelerometer data visualization tool that transforms your device's motion into beautiful terminal graphics.

<img src="assets/term.mp4" alt="Terminal Demo" width="600"/>

</div>

## Features

- Real-time accelerometer data capture from mobile devices
- Terminal-based UI with live graphs and gauges
- Historical data visualization with smooth scrolling
- Color-coded axis representation (X=Red, Y=Green, Z=Blue)
- Secure HTTPS communication
- iOS and Android support

## Demo


<div align="center">
  <video width="100%" controls>
    <source src="assets/term.mp4" type="video/mp4">
    Your browser does not support the video tag.
  </video>
</div>

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

## 📱 Usage

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
   -  Watch the real-time graphs and gauges update
   - ⌨️Press 'q' to quit the application



## ⌨️ Controls

- `q`: Quit the application
- Web interface automatically connects once permission is granted
