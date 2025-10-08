use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;

// Import weather database from parent module (server.rs)
use crate::{get_weather_database, WeatherData};

/// MCP Request structure
#[derive(Debug, Deserialize)]
pub struct McpWeatherRequest {
    pub cities: Vec<String>,
}

/// MCP Response structure - standardized format
#[derive(Debug, Serialize)]
pub struct McpWeatherResponse {
    pub tool: String,
    pub status: String,
    pub timestamp: String,
    pub results: HashMap<String, WeatherData>,
}

/// MCP Error Response
#[derive(Debug, Serialize)]
pub struct McpErrorResponse {
    pub tool: String,
    pub status: String,
    pub timestamp: String,
    pub error: String,
    pub code: u16,
}

/// MCP-compatible weather_info endpoint
///
/// This endpoint provides weather information in MCP (Model Context Protocol) format
/// for integration with Claude Code and other MCP-compatible clients.
///
/// ## Request Format
/// ```json
/// {
///   "cities": ["Gaza", "Stockholm", "Paris"]
/// }
/// ```
///
/// ## Response Format
/// ```json
/// {
///   "tool": "weather_info",
///   "status": "success",
///   "timestamp": "2025-10-08T14:30:00Z",
///   "results": {
///     "Gaza": {
///       "city": "Gaza",
///       "temperature": 27,
///       "condition": "Sunny",
///       "humidity": 60,
///       "wind_speed": 12
///     }
///   }
/// }
/// ```
pub async fn weather_info_mcp(
    Json(payload): Json<McpWeatherRequest>,
) -> Result<Json<McpWeatherResponse>, (StatusCode, Json<McpErrorResponse>)> {

    let timestamp = Utc::now().to_rfc3339();

    // Validation: check if cities list is empty
    if payload.cities.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(McpErrorResponse {
                tool: "weather_info".to_string(),
                status: "error".to_string(),
                timestamp,
                error: "Cities list cannot be empty".to_string(),
                code: 400,
            }),
        ));
    }

    // Validation: check if too many cities requested
    if payload.cities.len() > 20 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(McpErrorResponse {
                tool: "weather_info".to_string(),
                status: "error".to_string(),
                timestamp,
                error: format!(
                    "Too many cities requested. Maximum is 20, you requested {}",
                    payload.cities.len()
                ),
                code: 400,
            }),
        ));
    }

    println!("🔧 [MCP] Received weather_info request for {} cities", payload.cities.len());

    let weather_db = get_weather_database();
    let mut results = HashMap::new();

    for city in payload.cities {
        let city_lower = city.to_lowercase();

        let weather_data = if let Some((temp, condition, humidity, wind)) =
            weather_db.get(city_lower.as_str())
        {
            WeatherData {
                city: city.clone(),
                temperature: *temp,
                condition: condition.to_string(),
                humidity: *humidity,
                wind_speed: *wind,
            }
        } else {
            // Default data for unknown cities
            WeatherData {
                city: city.clone(),
                temperature: 20,
                condition: "Unknown".to_string(),
                humidity: 50,
                wind_speed: 10,
            }
        };

        println!("  ✓ [MCP] {} - {}°C, {}", city, weather_data.temperature, weather_data.condition);
        results.insert(city.clone(), weather_data);
    }

    println!("📤 [MCP] Sending response with {} results\n", results.len());

    Ok(Json(McpWeatherResponse {
        tool: "weather_info".to_string(),
        status: "success".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        results,
    }))
}

/// Health check endpoint for MCP service
pub async fn mcp_health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "service": "Rust Weather API - MCP Tool Provider",
        "status": "ok",
        "version": "0.3.0",
        "mcp_compatible": true,
        "tools": ["weather_info"],
        "endpoint": "/mcp/tool/weather_info"
    }))
}
