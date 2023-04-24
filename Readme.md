# Resource Sentinel

Resource Sentinel is a real-time CPU usage monitoring web application built using Rust and Preact. It utilizes Tokio for asynchronous programming and WebSockets for real-time communication between the server and the client.

## Features

- Real-time CPU usage monitoring for each core
- Responsive web interface
- Easy to set up and deploy

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Tokio](https://docs.rs/tokio/1.13.0/tokio/) (1.13.0 or later)

## Installation

1. Clone the repository:
   git clone https://github.com/yourusername/resource-sentinel.github

2. Change directory to the project folder:
   cd resource-sentinel

3. Build the project:
   cargo build --release

4. Run the compiled binary:
   ./target/release/resource-sentinel

5. Open your web browser and navigate to `http://localhost:3000`.

## Usage

The web application will display the live CPU usage for each core on your machine. The usage percentage is shown in real-time, and the display will be updated as new data is received from the server.

## Project Structure

The project consists of two main parts: a Rust server and a Preact client.

### Rust Server

The server is responsible for gathering CPU usage data and broadcasting it to all connected clients using WebSockets. The server is built using Axum, a web framework for Rust, and Tokio for asynchronous programming. The main components of the server are:

- `AppState`: Contains the state of the application, including the WebSocket sender.
- `main()`: The main entry point of the server. Sets up the router and starts the server.
- `root()`: Serves the main HTML file for the web application.
- `get_css()`: Serves the CSS file for the web application.
- `get_mjs()`: Serves the JavaScript (Preact) file for the web application.
- `realtime_cpu_get()`: Handles WebSocket upgrade requests.
- `realtime_cpu_stream()`: Streams CPU usage data to the connected WebSocket client.

### Preact Client

The client is responsible for rendering the user interface and updating the CPU usage display in real-time. The client is built using Preact, a lightweight JavaScript framework, and HTM for creating components using template literals. The main components of the client are:

- `App`: The main component that renders the CPU usage display.
- `html`: A helper function for creating Preact components using HTM template literals.
- WebSocket connection: Connects to the server to receive real-time CPU usage data.
