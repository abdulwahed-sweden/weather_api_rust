# 🦀 Rust Weather API v0.2.0

A high-performance, feature-rich weather API server and client built with Rust, Axum, and Tokio.

## ✨ Features

### Core Features
- **⚡ Fast HTTP Server**: Built with Axum web framework and async Tokio runtime
- **🌐 RESTful API**: Clean JSON-based weather information endpoints
- **🗄️ Rich Weather Data**: Temperature, humidity, wind speed, and conditions for 40+ cities
- **🔒 CORS Enabled**: Ready for frontend integration
- **🛡️ Type-Safe**: Leverages Rust's strong type system with Serde serialization
- **⚙️ Async Client**: Concurrent HTTP client with comprehensive error handling

### Advanced Features
- **🎨 Web Dashboard**: Beautiful, responsive single-page UI with Bootstrap 5
- **📊 Statistics Endpoint**: Get average temps, hottest/coldest cities, sortable data
- **✅ Request Validation**: Input validation with helpful error messages
- **🌍 40+ Cities**: Weather data for major cities worldwide
- **📈 Sorting**: Sort cities by temperature, name, humidity, or wind speed
- **🧪 Unit Tests**: Comprehensive test coverage for data integrity
- **📝 Logging**: HTTP request tracing middleware

## 📁 Project Structure

```
weather_api_rust/
├── Cargo.toml          # Project dependencies and metadata
├── index.html          # 🎨 Web Dashboard (Bootstrap 5 UI)
├── examples.sh         # cURL examples for testing
├── src/
│   ├── server.rs       # HTTP server with all endpoints + tests
│   └── client.rs       # Comprehensive test client
└── README.md           # This file
```

## 🛠️ Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)
- Optional: `jq` for pretty JSON output (`brew install jq`)

## 📦 Installation

```bash
# Clone or navigate to the project directory
cd weather_api_rust

# Build the project
cargo build --release

# Run tests
cargo test
```

## 🏃 Usage

### 1. Start the Server

```bash
cargo run --bin server
```

You should see:
```
🦀 Rust Weather API Server v0.2.0
====================================

🌤️  Starting server on http://localhost:3000
📡 Endpoints:
   GET  /           - Health check
   GET  /stats      - Weather statistics
   POST /weather    - Get weather info
   GET  /cities     - List all cities

✅ Server running! Press Ctrl+C to stop
```

### 2. Run the Test Client

```bash
cargo run --bin client
```

The client runs 4 comprehensive tests and displays detailed results.

### 3. Use the Web Dashboard

```bash
# Open the web dashboard in your browser
open index.html
```

**Features:**
- 🎨 Beautiful gradient design with purple/blue theme
- 📱 Fully responsive (works on mobile, tablet, desktop)
- 🌐 Real-time data from Rust API
- 🌤️ Dynamic weather icons based on conditions
- 📊 Global statistics sidebar
- ⚡ Smooth animations and loading states

### 4. Use cURL Examples

```bash
# Run all example commands
./examples.sh

# Or test individually:
curl http://localhost:3000/
curl http://localhost:3000/cities
curl http://localhost:3000/stats?sort=temp
```

## 🔌 API Endpoints

### Health Check
```http
GET http://localhost:3000/
```

**Response:**
```json
{
  "status": "ok",
  "service": "Rust Weather API",
  "version": "0.2.0",
  "endpoints": [
    "GET /",
    "GET /stats",
    "GET /cities",
    "POST /weather"
  ]
}
```

### Get All Cities
```http
GET http://localhost:3000/cities
```

**Response:**
```json
{
  "count": 40,
  "cities": ["amsterdam", "athens", "auckland", ...]
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

**Response:**
```json
{
  "data": {
    "Stockholm": {
      "city": "Stockholm",
      "temperature": 15,
      "condition": "Cloudy",
      "humidity": 75,
      "wind_speed": 15
    },
    "Gaza": {
      "city": "Gaza",
      "temperature": 27,
      "condition": "Sunny",
      "humidity": 60,
      "wind_speed": 12
    }
  }
}
```

**Request Validation:**
- ❌ Empty cities array: Returns 400 error
- ❌ More than 20 cities: Returns 400 error with message
- ✅ Unknown cities: Returns default values (20°C, Unknown condition)

### Get Weather Statistics
```http
GET http://localhost:3000/stats?sort=temp
```

**Query Parameters:**
- `sort=temp` or `sort=temperature` - Sort by temperature
- `sort=name` or `sort=city` - Sort alphabetically
- `sort=humidity` - Sort by humidity
- `sort=wind` - Sort by wind speed

**Response:**
```json
{
  "total_cities": 40,
  "average_temp": 21.4,
  "hottest_city": "riyadh",
  "coldest_city": "moscow",
  "cities": [
    {
      "city": "moscow",
      "temperature": 8,
      "condition": "Snowy",
      "humidity": 90,
      "wind_speed": 25
    },
    ...
  ]
}
```

## 🌍 Supported Cities (40+)

The API includes comprehensive weather data for:

**Europe:** Stockholm, Paris, London, Berlin, Moscow, Madrid, Rome, Amsterdam, Vienna, Athens, Istanbul

**Middle East & Africa:** Gaza, Dubai, Cairo, Riyadh, Nairobi, Cape Town, Johannesburg

**Asia:** Tokyo, Bangkok, Singapore, Mumbai, Delhi, Beijing, Shanghai, Seoul

**Americas:** New York, Los Angeles, San Francisco, Chicago, Toronto, Vancouver, Mexico City, Buenos Aires, São Paulo, Rio de Janeiro

**Oceania:** Sydney, Melbourne, Auckland, Wellington

*Unknown cities default to 20°C with "Unknown" conditions*

## 🧪 Testing

### Run Unit Tests
```bash
cargo test
```

Tests include:
- ✅ Database integrity
- ✅ Data format validation
- ✅ Value range checks
- ✅ Minimum city count

### Run Integration Tests
```bash
# Terminal 1: Start server
cargo run --bin server

# Terminal 2: Run client tests
cargo run --bin client

# Terminal 3: Run cURL examples
./examples.sh
```

## 🔧 Development

### Build for Development
```bash
cargo build
```

### Build for Production
```bash
cargo build --release
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

## 📚 Dependencies

- **axum** `0.7` - Web framework
- **tokio** `1.41` - Async runtime
- **serde** `1.0` - Serialization/deserialization
- **serde_json** `1.0` - JSON support
- **tower** `0.5` - Middleware and utilities
- **tower-http** `0.6` - HTTP middleware (CORS, tracing)
- **reqwest** `0.12` - HTTP client

## 🚀 Performance

Built with Rust for maximum performance:
- **Memory Safety**: Zero-cost abstractions, no garbage collection
- **Concurrency**: Async/await with Tokio handles thousands of requests
- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Speed**: Near C/C++ performance with modern ergonomics

## 📝 License

MIT License - Free to use for learning and development!

## 🤝 Contributing

Contributions welcome! Feel free to open issues or submit pull requests.

## 🎯 Roadmap

- [ ] Integration with real weather APIs
- [ ] Database persistence (PostgreSQL/SQLite)
- [ ] Authentication & API keys
- [ ] Rate limiting
- [ ] Caching layer (Redis)
- [ ] Docker containerization
- [ ] Kubernetes deployment manifests

---

**Built with 🦀 Rust** | **Powered by Axum & Tokio** | Made with ❤️ for the community
