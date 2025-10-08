# 🔧 MCP Setup Guide for Claude Desktop

This guide explains how to integrate the Rust Weather API with Claude Desktop using the Model Context Protocol (MCP).

## Overview

The Rust Weather API uses a **two-component architecture** for Claude Desktop integration:

1. **HTTP Server** (`cargo run --bin server`) - Rust-based weather API on port 3000
2. **MCP stdio Bridge** (`mcp_stdio_bridge.py`) - Python script that translates between:
   - Claude Desktop's stdio JSON-RPC protocol
   - The Rust HTTP server's REST API

## Why Two Components?

Claude Desktop communicates with MCP servers using **stdio (standard input/output)** and the JSON-RPC protocol. However, our Rust Weather API is an HTTP-based REST server. The Python bridge acts as a translator between these two protocols.

```
┌──────────────────┐
│  Claude Desktop  │
│   (stdio/JSON)   │
└────────┬─────────┘
         │ JSON-RPC over stdio
         ▼
┌──────────────────┐
│ mcp_stdio_bridge │
│   (Python)       │
└────────┬─────────┘
         │ HTTP/REST
         ▼
┌──────────────────┐
│  Rust Server     │
│  (Port 3000)     │
└──────────────────┘
```

## 📋 Prerequisites

1. **Python 3** with `requests` library:
   ```bash
   python3 -m pip install requests
   ```

2. **Rust Weather API** running on port 3000:
   ```bash
   cd weather_api_rust
   cargo run --bin server
   ```

## 🚀 Setup Steps

### 1. Install Python Dependencies

```bash
python3 -m pip install requests
```

### 2. Start the Rust HTTP Server

In one terminal, keep the Rust server running:

```bash
cd ~/weather_api_rust
cargo run --bin server
```

You should see:
```
🦀 Rust Weather API Server v0.3.0 - MCP Edition
================================================
🌤️  Starting server on http://localhost:3000
...
✅ Server running!
```

### 3. Configure Claude Desktop

Edit your Claude Desktop config file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`

Add the following MCP server entry:

```json
{
  "mcpServers": {
    "rust-weather-api": {
      "command": "/usr/local/bin/python3",
      "args": [
        "/Users/YOUR_USERNAME/weather_api_rust/mcp_stdio_bridge.py"
      ],
      "description": "Rust Weather API MCP server via stdio bridge (requires HTTP server on port 3000)"
    }
  }
}
```

**Important:** Replace `/Users/YOUR_USERNAME/` with your actual home directory path.

### 4. Restart Claude Desktop

1. Quit Claude Desktop completely (Cmd+Q on macOS)
2. Reopen Claude Desktop
3. The MCP server should appear in your Local MCP servers list

### 5. Verify Connection

Check that the server shows as **connected** (not failed) in Claude Desktop's MCP server list.

## 🧪 Testing the Integration

Once connected, you can use the weather_info tool:

```
Get weather for Gaza, Stockholm, and Paris
```

Or directly:
```
Show me weather information for London, Tokyo, and New York
```

Expected response format:
```
Weather Information (Retrieved: 2025-10-08T15:21:21.543324+00:00)
============================================================

🌍 Gaza
   Temperature: 27°C
   Condition: Sunny
   Humidity: 60%
   Wind Speed: 12 km/h

🌍 Stockholm
   Temperature: 15°C
   Condition: Cloudy
   Humidity: 75%
   Wind Speed: 15 km/h
...
```

## 🐛 Troubleshooting

### Server Shows "failed" in Claude Desktop

**Symptom:** The rust-weather-api shows "failed" with "Server disconnected" error.

**Solutions:**

1. **Check if Rust server is running:**
   ```bash
   curl http://localhost:3000/mcp
   ```
   Should return JSON with `"status": "ok"`

2. **Check if port 3000 is available:**
   ```bash
   lsof -i :3000
   ```

3. **Test the Python bridge manually:**
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
     python3 /Users/YOUR_USERNAME/weather_api_rust/mcp_stdio_bridge.py
   ```
   Should return JSON-RPC response

4. **Check Python path in config:**
   ```bash
   which python3
   ```
   Update the `command` in config to match this path

5. **Verify requests library is installed:**
   ```bash
   python3 -c "import requests; print('OK')"
   ```

### HTTP Server Not Responding

**Symptom:** Bridge shows error "Cannot connect to Rust Weather API server"

**Solution:**
```bash
cd ~/weather_api_rust
cargo run --bin server
```

Keep this running in a separate terminal.

### Port 3000 Already in Use

**Symptom:** Server fails to start with "address already in use"

**Solution:**
```bash
lsof -ti:3000 | xargs kill -9
cargo run --bin server
```

## 📊 How It Works

### MCP stdio Bridge (`mcp_stdio_bridge.py`)

The Python bridge implements the MCP protocol:

1. **Reads JSON-RPC requests** from stdin (from Claude Desktop)
2. **Translates** to HTTP requests to the Rust server
3. **Formats responses** according to MCP specification
4. **Writes JSON-RPC responses** to stdout (back to Claude Desktop)

### Supported MCP Methods

- `initialize` - Initializes the MCP session
- `tools/list` - Lists available tools (weather_info)
- `tools/call` - Calls the weather_info tool
- `ping` - Health check

### Weather Info Tool

**Name:** `weather_info`

**Input Schema:**
```json
{
  "cities": ["string", "string", ...]
}
```

**Constraints:**
- Maximum 20 cities per request
- Cities must be non-empty array
- Unknown cities return default values (20°C, "Unknown")

## 🔗 Related Files

- `mcp_stdio_bridge.py` - MCP stdio protocol bridge
- `src/mcp_api.rs` - Rust HTTP MCP endpoints
- `src/server.rs` - Main Rust server
- `CLAUDE.md` - HTTP API documentation
- `README.md` - Project overview

## 📝 Notes

- The Rust HTTP server must be running **before** Claude Desktop starts the MCP bridge
- The bridge automatically checks server connectivity on startup
- All MCP communication goes through the bridge (Claude Desktop never talks directly to the HTTP server)
- The HTTP server on port 3000 remains accessible for direct API calls and web dashboard

## ✅ Success Indicators

Your setup is working correctly when:

1. ✅ Rust server shows "Server running!" message
2. ✅ curl to `http://localhost:3000/mcp` returns JSON
3. ✅ Claude Desktop shows rust-weather-api as **connected** (not failed)
4. ✅ Weather queries in Claude return formatted results
5. ✅ No "Server disconnected" errors in Claude Desktop

---

**Need Help?** Check the main README.md or open an issue on GitHub.
