# 🦀 Rust Weather API v0.3.0

A high-performance, MCP-compatible weather API server and client built with Rust, Axum, and Tokio. Fully integrated with Claude Code Desktop via Model Context Protocol (MCP).

## ✨ Features

### Core Features

- **⚡ Fast HTTP Server**: Built with Axum web framework and async Tokio runtime
- **🌐 RESTful API**: Clean JSON-based weather information endpoints
- **🗄️ Rich Weather Data**: Temperature, humidity, wind speed, and conditions for 40+ cities
- **🔒 CORS Enabled**: Ready for frontend integration
- **🛡️ Type-Safe**: Leverages Rust's strong type system with Serde serialization
- **⚙️ Async Client**: Concurrent HTTP client with comprehensive error handling

### MCP Integration (NEW in v0.3.0)

- **🤖 Claude Code Compatible**: Full Model Context Protocol integration
- **🔧 MCP Tool Provider**: `/mcp/tool/weather_info` endpoint for AI assistant integration
- **📋 CLAUDE.md Manifest**: Complete tool documentation for Claude Code Desktop
- **⏱️ Standardized Responses**: ISO 8601 timestamps and MCP-compliant JSON format
- **✅ Tool Validation**: Request validation with proper error codes (400, 500)
- **🌐 Zero Configuration**: Auto-detected by Claude Code Desktop on localhost:3000

### Advanced Features

- **🎨 Web Dashboard**: Clean Scandinavian minimal design with Bootstrap 5
- **📊 Statistics Endpoint**: Get average temps, hottest/coldest cities, sortable data
- **✅ Request Validation**: Input validation with helpful error messages
- **🌍 40+ Cities**: Weather data for major cities worldwide
- **📈 Sorting**: Sort cities by temperature, name, humidity, or wind speed
- **🧪 Unit Tests**: Comprehensive test coverage for data integrity
- **📝 Logging**: HTTP request tracing middleware
- **🔗 MCP Badge**: Live connection indicator on web dashboard

## 📁 Project Structure

```
weather_api_rust/
├── Cargo.toml          # Project dependencies and metadata
├── index.html          # 🎨 Web Dashboard (Clean Scandinavian minimal design)
├── CLAUDE.md           # 🤖 MCP Tool Manifest for Claude Code integration
├── examples.sh         # cURL examples for testing
├── src/
│   ├── server.rs       # HTTP server with all endpoints + tests
│   ├── mcp_api.rs      # 🔧 MCP Tool Provider module (NEW in v0.3.0)
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
🦀 Rust Weather API Server v0.3.0 - MCP Edition
================================================

🌤️  Starting server on http://localhost:3000
📡 Standard API Endpoints:
   GET  /           - Health check
   GET  /stats      - Weather statistics
   POST /weather    - Get weather info
   GET  /cities     - List all cities

🔧 MCP Tool Provider Endpoints:
   GET  /mcp        - MCP health check
   POST /mcp/tool/weather_info - MCP weather tool

🤖 Claude Code Integration: ENABLED

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

- 🎨 Clean Scandinavian minimal design (blue/white theme)
- 📱 Fully responsive (works on mobile, tablet, desktop)
- 🌐 Real-time data from Rust API
- 🌤️ Dynamic weather icons based on conditions
- 📊 Global statistics panel
- 🔗 Live MCP connection indicator with glowing dot
- ⚡ Smooth animations and loading states
- 💫 Professional, calm aesthetic with whitespace

### 4. Use with Claude Code Desktop

Once the server is running, Claude Code Desktop will automatically detect the MCP tool:

```
You: Get weather for Gaza, Stockholm, and Paris
```

Claude Code will automatically:

1. Detect the local MCP tool endpoint at `http://localhost:3000/mcp/tool/weather_info`
2. Call the endpoint with the cities list
3. Parse and present the weather information

See `CLAUDE.md` for complete MCP integration documentation.

### 5. Use cURL Examples

```bash
# Run all example commands
./examples.sh

# Or test individually:
curl http://localhost:3000/
curl http://localhost:3000/cities
curl http://localhost:3000/stats?sort=temp
```

## 🔌 API Endpoints

### MCP Health Check (NEW in v0.3.0)

```http
GET http://localhost:3000/mcp
```

**Response:**

```json
{
  "service": "Rust Weather API - MCP Tool Provider",
  "status": "ok",
  "version": "0.3.0",
  "mcp_compatible": true,
  "tools": ["weather_info"],
  "endpoint": "/mcp/tool/weather_info"
}
```

### MCP Weather Tool (NEW in v0.3.0)

```http
POST http://localhost:3000/mcp/tool/weather_info
Content-Type: application/json

{
  "cities": ["Gaza", "Stockholm", "Paris"]
}
```

**Response:**

```json
{
  "tool": "weather_info",
  "status": "success",
  "timestamp": "2025-10-08T14:30:00Z",
  "results": {
    "Gaza": {
      "city": "Gaza",
      "temperature": 27,
      "condition": "Sunny",
      "humidity": 60,
      "wind_speed": 12
    },
    "Stockholm": {
      "city": "Stockholm",
      "temperature": 15,
      "condition": "Cloudy",
      "humidity": 75,
      "wind_speed": 15
    }
  }
}
```

**Validation:**

- ❌ Empty cities array → Returns 400 error
- ❌ More than 20 cities → Returns 400 error
- ✅ Unknown cities → Returns default values (20°C, Unknown condition)

---

### Health Check

```http
GET http://localhost:3000/
```

**Response:**

```json
{
  "status": "ok",
  "service": "Rust Weather API",
  "version": "0.3.0",
  "mcp_enabled": true,
  "endpoints": [
    "GET /",
    "GET /stats",
    "GET /cities",
    "POST /weather",
    "GET /mcp",
    "POST /mcp/tool/weather_info"
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

_Unknown cities default to 20°C with "Unknown" conditions_

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
- **chrono** `0.4` - Date/time handling for MCP timestamps (NEW in v0.3.0)

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

- [x] Model Context Protocol (MCP) integration ✅ v0.3.0
- [x] Claude Code Desktop compatibility ✅ v0.3.0
- [x] Clean minimal web dashboard ✅ v0.3.0
- [ ] Integration with real weather APIs (OpenWeatherMap, WeatherAPI)
- [ ] Database persistence (PostgreSQL/SQLite)
- [ ] Authentication & API keys
- [ ] Rate limiting
- [ ] Caching layer (Redis)
- [ ] Docker containerization
- [ ] Kubernetes deployment manifests
- [ ] Additional MCP tools (forecasts, historical data)

---

## 🔗 Related Documentation

- **MCP Integration Guide**: See `CLAUDE.md` for complete Model Context Protocol documentation
- **MCP Specification**: https://modelcontextprotocol.io
- **Claude Code**: https://claude.com/claude-code
- **GitHub Repository**: https://github.com/abdulwahed-sweden/weather_api_rust

---

**Built with 🦀 Rust v0.3.0** | **Powered by Axum & Tokio** | **MCP-Compatible** | Made with ❤️ for the community.
