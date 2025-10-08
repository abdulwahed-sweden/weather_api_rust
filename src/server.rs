use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct WeatherRequest {
    cities: Vec<String>,
}

#[derive(Debug, Serialize)]
struct WeatherResponse {
    data: HashMap<String, String>,
}

/// Main entry point for the weather API server
#[tokio::main]
async fn main() {
    println!("🦀 Rust Weather API Server");
    println!("================================");
    println!();
    println!("🌤️  Starting server on http://localhost:3000");
    println!("📡 Endpoints:");
    println!("   GET  / - Health check");
    println!("   POST /weather - Get weather info");
    println!();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/weather", post(get_weather))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("✅ Server running! Press Ctrl+C to stop\n");

    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "Weather API",
        "version": "0.1.0"
    }))
}

/// Get weather information for multiple cities
async fn get_weather(
    Json(payload): Json<WeatherRequest>,
) -> Result<Json<WeatherResponse>, StatusCode> {
    println!("📥 Received weather request for {} cities", payload.cities.len());

    // Mock temperature data - in a real app, this would call a weather API
    let mock_temps: HashMap<&str, i32> = [
        ("stockholm", 15),
        ("gaza", 27),
        ("paris", 19),
        ("london", 12),
        ("new york", 18),
        ("tokyo", 22),
        ("sydney", 24),
        ("berlin", 14),
        ("moscow", 8),
        ("dubai", 35),
        ("cairo", 30),
        ("riyadh", 38),
    ]
    .iter()
    .cloned()
    .collect();

    let mut response_data = HashMap::new();

    for city in payload.cities {
        let city_lower = city.to_lowercase();
        let temperature = mock_temps.get(city_lower.as_str()).unwrap_or(&20);

        let weather_info = format!("The temperature in {} is {}°C", city, temperature);
        response_data.insert(city.clone(), weather_info);

        println!("  ✓ {}", city);
    }

    println!("📤 Sending response\n");

    Ok(Json(WeatherResponse {
        data: response_data,
    }))
}
