#!/usr/bin/env python3
"""
MCP stdio bridge for Rust Weather API
Bridges between Claude Desktop's stdio MCP protocol and the HTTP-based Rust server
"""

import sys
import json
import requests
import asyncio
from typing import Any, Dict

# HTTP server endpoint
HTTP_SERVER = "http://localhost:3000"

def log_error(message: str):
    """Log errors to stderr"""
    print(f"ERROR: {message}", file=sys.stderr, flush=True)

def send_response(response: Dict[str, Any]):
    """Send JSON-RPC response to stdout"""
    json.dump(response, sys.stdout)
    sys.stdout.write("\n")
    sys.stdout.flush()

def handle_initialize(request_id: Any):
    """Handle MCP initialize request"""
    send_response({
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "rust-weather-api",
                "version": "0.3.0"
            }
        }
    })

def handle_list_tools(request_id: Any):
    """Handle tools/list request"""
    send_response({
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {
            "tools": [
                {
                    "name": "weather_info",
                    "description": "Get weather information for specified cities from Rust Weather API",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "cities": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "List of city names (max 20)"
                            }
                        },
                        "required": ["cities"]
                    }
                }
            ]
        }
    })

def handle_call_tool(request_id: Any, params: Dict[str, Any]):
    """Handle tools/call request"""
    tool_name = params.get("name")
    arguments = params.get("arguments", {})

    if tool_name != "weather_info":
        send_response({
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {
                "code": -32601,
                "message": f"Unknown tool: {tool_name}"
            }
        })
        return

    # Call the HTTP endpoint
    try:
        response = requests.post(
            f"{HTTP_SERVER}/mcp/tool/weather_info",
            json=arguments,
            headers={"Content-Type": "application/json"},
            timeout=10
        )

        if response.status_code == 200:
            data = response.json()

            # Format results as text
            results_text = []
            results_text.append(f"Weather Information (Retrieved: {data['timestamp']})")
            results_text.append("=" * 60)

            for city, info in data['results'].items():
                results_text.append(f"\n🌍 {city}")
                results_text.append(f"   Temperature: {info['temperature']}°C")
                results_text.append(f"   Condition: {info['condition']}")
                results_text.append(f"   Humidity: {info['humidity']}%")
                results_text.append(f"   Wind Speed: {info['wind_speed']} km/h")

            send_response({
                "jsonrpc": "2.0",
                "id": request_id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": "\n".join(results_text)
                        }
                    ]
                }
            })
        else:
            error_data = response.json()
            send_response({
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {
                    "code": response.status_code,
                    "message": error_data.get('error', 'HTTP request failed')
                }
            })

    except requests.exceptions.ConnectionError:
        send_response({
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {
                "code": -32603,
                "message": "Cannot connect to Rust Weather API server. Please ensure 'cargo run --bin server' is running on port 3000."
            }
        })
    except Exception as e:
        log_error(f"Error calling HTTP endpoint: {str(e)}")
        send_response({
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {
                "code": -32603,
                "message": f"Internal error: {str(e)}"
            }
        })

def handle_ping(request_id: Any):
    """Handle ping request"""
    send_response({
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {}
    })

def main():
    """Main event loop - read JSON-RPC requests from stdin"""
    log_error("MCP stdio bridge started")

    # Check if HTTP server is available
    try:
        response = requests.get(f"{HTTP_SERVER}/mcp", timeout=2)
        if response.status_code == 200:
            log_error("Successfully connected to Rust Weather API HTTP server")
        else:
            log_error("WARNING: Rust Weather API HTTP server returned unexpected status")
    except:
        log_error("WARNING: Cannot connect to Rust Weather API HTTP server on port 3000")
        log_error("Please start the server with: cargo run --bin server")

    # Process stdin line by line
    for line in sys.stdin:
        try:
            line = line.strip()
            if not line:
                continue

            request = json.loads(line)
            request_id = request.get("id")
            method = request.get("method")
            params = request.get("params", {})

            log_error(f"Received request: {method}")

            if method == "initialize":
                handle_initialize(request_id)
            elif method == "tools/list":
                handle_list_tools(request_id)
            elif method == "tools/call":
                handle_call_tool(request_id, params)
            elif method == "ping":
                handle_ping(request_id)
            elif method == "notifications/initialized":
                # Just acknowledge, no response needed for notifications
                pass
            else:
                send_response({
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32601,
                        "message": f"Method not found: {method}"
                    }
                })

        except json.JSONDecodeError as e:
            log_error(f"Invalid JSON: {str(e)}")
        except Exception as e:
            log_error(f"Error processing request: {str(e)}")

if __name__ == "__main__":
    main()
