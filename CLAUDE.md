# 🤖 Claude Code MCP Integration

## Rust Weather API - MCP Tool Provider

This document describes the Model Context Protocol (MCP) integration for the Rust Weather API, enabling Claude Code Desktop to interact directly with this weather service.

---

## 🔧 MCP Tools Available

### `weather_info`

**Description:** Returns detailed weather information for specified cities using the Rust Weather API backend.

**Endpoint:** `http://localhost:3000/mcp/tool/weather_info`

**Method:** `POST`

**Request Format:**
```json
{
  "cities": ["Gaza", "Stockholm", "Paris"]
}
```

**Response Format:**
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
    },
    "Paris": {
      "city": "Paris",
      "temperature": 19,
      "condition": "Rainy",
      "humidity": 80,
      "wind_speed": 18
    }
  }
}
```

**Validation Rules:**
- ❌ Empty cities array → Returns 400 error
- ❌ More than 20 cities → Returns 400 error
- ✅ Unknown cities → Returns default values (20°C, Unknown condition)

**Error Response:**
```json
{
  "tool": "weather_info",
  "status": "error",
  "timestamp": "2025-10-08T14:30:00Z",
  "error": "Cities list cannot be empty",
  "code": 400
}
```

---

## 🚀 How to Use with Claude Code

### 1. Start the Rust Server
```bash
cd weather_api_rust
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

### 2. Using the Tool in Claude Code

Once the server is running, you can use the weather tool in your conversations:

```
Get weather for Gaza, Stockholm, and Paris
```

Claude Code will automatically:
1. Detect the local MCP tool endpoint
2. Call `http://localhost:3000/mcp/tool/weather_info`
3. Parse the response
4. Present the weather information

---

## 📊 Available Cities

The tool supports 40+ cities worldwide:

- **Europe:** Stockholm, Paris, London, Berlin, Moscow, Madrid, Rome, Amsterdam, Vienna, Athens, Istanbul
- **Middle East & Africa:** Gaza, Dubai, Cairo, Riyadh, Nairobi, Cape Town, Johannesburg
- **Asia:** Tokyo, Bangkok, Singapore, Mumbai, Delhi, Beijing, Shanghai, Seoul
- **Americas:** New York, Los Angeles, San Francisco, Chicago, Toronto, Vancouver, Mexico City, Buenos Aires, São Paulo, Rio de Janeiro
- **Oceania:** Sydney, Melbourne, Auckland, Wellington

*Unknown cities will return default values (20°C, Unknown condition)*

---

## 🔍 Testing the MCP Endpoint

### Health Check
```bash
curl http://localhost:3000/mcp
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

### Direct Tool Call
```bash
curl -X POST http://localhost:3000/mcp/tool/weather_info \
  -H "Content-Type: application/json" \
  -d '{"cities": ["Gaza", "Stockholm", "Paris"]}'
```

---

## 🏗️ Architecture

```
┌─────────────────┐
│  Claude Code    │
│    Desktop      │
└────────┬────────┘
         │ MCP Protocol
         │ (HTTP POST)
         ▼
┌─────────────────┐
│  Rust Weather   │
│  API Server     │
│  (Port 3000)    │
├─────────────────┤
│ MCP Endpoint:   │
│ /mcp/tool/      │
│ weather_info    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Weather Data    │
│ (40+ Cities)    │
└─────────────────┘
```

---

## ⚙️ Configuration

### Server Configuration
- **Port:** 3000
- **CORS:** Enabled for all origins
- **Request Tracing:** Enabled
- **Max Cities per Request:** 20

### MCP Response Format
- **Standard Fields:** `tool`, `status`, `timestamp`
- **Success Status:** `"success"`
- **Error Status:** `"error"`
- **Timestamp Format:** ISO 8601 / RFC 3339

---

## 🛠️ Development

### Adding New MCP Tools

1. Create endpoint in `src/mcp_api.rs`
2. Add route in `src/server.rs`
3. Update this manifest file
4. Document in README.md

### Testing

```bash
# Run server tests
cargo test

# Run integration tests
cargo run --bin client

# Test MCP endpoint
curl http://localhost:3000/mcp
```

---

## 📝 License

MIT License - Free to use for development and integration.

---

## 🔗 Links

- **GitHub Repository:** https://github.com/abdulwahed-sweden/weather_api_rust
- **API Documentation:** See README.md
- **MCP Specification:** https://modelcontextprotocol.io

---

**Built with 🦀 Rust | MCP-Compatible | Claude Code Ready**
