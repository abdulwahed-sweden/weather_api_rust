# 🦀 Rust Weather API

A high-performance weather API server and client built with Rust, Axum, and Tokio.

## 🚀 Features

- **Fast HTTP Server**: Built with Axum web framework and async Tokio runtime
- **RESTful API**: Clean JSON-based weather information endpoints
- **Mock Weather Data**: Returns temperature information for multiple cities
- **CORS Enabled**: Ready for frontend integration
- **Type-Safe**: Leverages Rust's strong type system with Serde serialization
- **Concurrent Client**: Async HTTP client with error handling

## 📁 Project Structure

```
weather_api_rust/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── server.rs       # HTTP server with weather endpoints
│   └── client.rs       # HTTP client for testing
└── README.md           # This file
```

## 🛠️ Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)

## 📦 Installation

```bash
# Clone or navigate to the project directory
cd weather_api_rust

# Build the project
cargo build --release
```

## 🏃 Usage

### 1. Start the Server

```bash
cargo run --bin server
```

You should see:
```
🦀 Rust Weather API Server
================================

🌤️  Starting server on http://localhost:3000
📡 Endpoints:
   GET  / - Health check
   POST /weather - Get weather info

✅ Server running! Press Ctrl+C to stop
```

### 2. Run the Client (in another terminal)

```bash
cargo run --bin client
```

Expected output:
```
🦀 Rust Weather API Client
================================

🔍 Checking if server is ready...
✅ Server is ready!

🌍 Requesting weather for: Stockholm, Gaza, Paris
--------------------------------------------------

📊 Weather Results:

   The temperature in Gaza is 27°C
   The temperature in Paris is 19°C
   The temperature in Stockholm is 15°C

✅ Request completed successfully!
```

## 🔌 API Endpoints

### Health Check
```http
GET http://localhost:3000/
```

Response:
```json
{
  "status": "ok",
  "service": "Weather API",
  "version": "0.1.0"
}
```

### Get Weather Information
```http
POST http://localhost:3000/weather
Content-Type: application/json

{
  "cities": ["Stockholm", "Gaza", "Paris"]
}
```

Response:
```json
{
  "data": {
    "Stockholm": "The temperature in Stockholm is 15°C",
    "Gaza": "The temperature in Gaza is 27°C",
    "Paris": "The temperature in Paris is 19°C"
  }
}
```

## 🌍 Supported Cities

The server includes mock data for:
- Stockholm (15°C)
- Gaza (27°C)
- Paris (19°C)
- London (12°C)
- New York (18°C)
- Tokyo (22°C)
- Sydney (24°C)
- Berlin (14°C)
- Moscow (8°C)
- Dubai (35°C)
- Cairo (30°C)
- Riyadh (38°C)

*Default: 20°C for unknown cities*

## 🧪 Testing with cURL

```bash
# Health check
curl http://localhost:3000/

# Get weather
curl -X POST http://localhost:3000/weather \
  -H "Content-Type: application/json" \
  -d '{"cities": ["Stockholm", "Gaza", "Paris"]}'
```

## 🔧 Development

### Build for Development
```bash
cargo build
```

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Check for Errors
```bash
cargo clippy
```

## 📚 Dependencies

- **axum**: Web framework for Rust
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **serde_json**: JSON support
- **tower**: Middleware and utilities
- **tower-http**: HTTP-specific middleware (CORS)
- **reqwest**: HTTP client

## 🚀 Performance

Built with Rust for:
- **Memory Safety**: No garbage collection, zero-cost abstractions
- **Concurrency**: Async/await with Tokio for handling thousands of concurrent requests
- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Speed**: Near C/C++ performance with modern ergonomics

## 📝 License

MIT License - feel free to use this project for learning and development!

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---

**Built with 🦀 Rust** | Made with ❤️ for the community
