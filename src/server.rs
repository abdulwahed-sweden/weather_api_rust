use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Debug, Deserialize)]
struct WeatherRequest {
    cities: Vec<String>,
}

#[derive(Debug, Serialize)]
struct WeatherResponse {
    data: HashMap<String, WeatherData>,
}

#[derive(Debug, Serialize, Clone)]
struct WeatherData {
    city: String,
    temperature: i32,
    condition: String,
    humidity: i32,
    wind_speed: i32,
}

#[derive(Debug, Deserialize)]
struct StatsQuery {
    #[serde(default)]
    sort: String,
}

#[derive(Debug, Serialize)]
struct StatsResponse {
    total_cities: usize,
    average_temp: f32,
    hottest_city: String,
    coldest_city: String,
    cities: Vec<WeatherData>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

/// Main entry point for the weather API server
#[tokio::main]
async fn main() {
    println!("🦀 Rust Weather API Server v0.2.0");
    println!("====================================");
    println!();
    println!("🌤️  Starting server on http://localhost:3000");
    println!("📡 Endpoints:");
    println!("   GET  /           - Health check");
    println!("   GET  /stats      - Weather statistics");
    println!("   POST /weather    - Get weather info");
    println!("   GET  /cities     - List all cities");
    println!();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/weather", post(get_weather))
        .route("/stats", get(get_stats))
        .route("/cities", get(get_cities))
        .layer(TraceLayer::new_for_http())
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
        "service": "Rust Weather API",
        "version": "0.2.0",
        "endpoints": [
            "GET /",
            "GET /stats",
            "GET /cities",
            "POST /weather"
        ]
    }))
}

/// Get database of all cities with weather data
fn get_weather_database() -> HashMap<&'static str, (i32, &'static str, i32, i32)> {
    // (temperature, condition, humidity, wind_speed)
    [
        ("stockholm", (15, "Cloudy", 75, 15)),
        ("gaza", (27, "Sunny", 60, 12)),
        ("paris", (19, "Rainy", 80, 18)),
        ("london", (12, "Foggy", 85, 20)),
        ("new york", (18, "Partly Cloudy", 70, 22)),
        ("tokyo", (22, "Clear", 65, 10)),
        ("sydney", (24, "Sunny", 55, 14)),
        ("berlin", (14, "Overcast", 78, 16)),
        ("moscow", (8, "Snowy", 90, 25)),
        ("dubai", (35, "Hot & Sunny", 45, 8)),
        ("cairo", (30, "Sunny", 40, 10)),
        ("riyadh", (38, "Very Hot", 35, 15)),
        ("madrid", (21, "Sunny", 50, 12)),
        ("rome", (23, "Clear", 60, 11)),
        ("amsterdam", (13, "Rainy", 82, 19)),
        ("vienna", (16, "Cloudy", 70, 14)),
        ("athens", (26, "Sunny", 55, 13)),
        ("istanbul", (20, "Partly Cloudy", 68, 16)),
        ("bangkok", (32, "Hot & Humid", 85, 9)),
        ("singapore", (31, "Tropical", 80, 8)),
        ("mumbai", (29, "Humid", 75, 14)),
        ("delhi", (28, "Hazy", 65, 12)),
        ("beijing", (17, "Smoggy", 60, 15)),
        ("shanghai", (21, "Rainy", 78, 17)),
        ("seoul", (19, "Clear", 62, 13)),
        ("los angeles", (24, "Sunny", 50, 10)),
        ("san francisco", (18, "Foggy", 72, 16)),
        ("chicago", (15, "Windy", 68, 28)),
        ("toronto", (13, "Cloudy", 70, 18)),
        ("vancouver", (14, "Rainy", 85, 12)),
        ("mexico city", (22, "Sunny", 45, 11)),
        ("buenos aires", (20, "Pleasant", 68, 14)),
        ("sao paulo", (25, "Partly Cloudy", 70, 13)),
        ("rio de janeiro", (28, "Hot & Humid", 75, 12)),
        ("cape town", (21, "Windy", 65, 24)),
        ("johannesburg", (23, "Sunny", 50, 14)),
        ("nairobi", (24, "Warm", 55, 12)),
        ("melbourne", (19, "Variable", 68, 20)),
        ("auckland", (17, "Rainy", 80, 22)),
        ("wellington", (15, "Windy", 75, 30)),
    ]
    .iter()
    .cloned()
    .collect()
}

/// Get weather information for multiple cities
async fn get_weather(
    Json(payload): Json<WeatherRequest>,
) -> Result<Json<WeatherResponse>, (StatusCode, Json<ErrorResponse>)> {
    println!("📥 Received weather request for {} cities", payload.cities.len());

    // Validation: check if cities list is empty
    if payload.cities.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Cities list cannot be empty".to_string(),
                code: 400,
            }),
        ));
    }

    // Validation: check if too many cities requested
    if payload.cities.len() > 20 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!(
                    "Too many cities requested. Maximum is 20, you requested {}",
                    payload.cities.len()
                ),
                code: 400,
            }),
        ));
    }

    let weather_db = get_weather_database();
    let mut response_data = HashMap::new();

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

        println!("  ✓ {} - {}°C, {}", city, weather_data.temperature, weather_data.condition);
        response_data.insert(city.clone(), weather_data);
    }

    println!("📤 Sending response\n");

    Ok(Json(WeatherResponse {
        data: response_data,
    }))
}

/// Get statistics about all weather data
async fn get_stats(Query(params): Query<StatsQuery>) -> impl IntoResponse {
    println!("📊 Received stats request");

    let weather_db = get_weather_database();
    let mut cities_data: Vec<WeatherData> = weather_db
        .iter()
        .map(|(name, (temp, condition, humidity, wind))| WeatherData {
            city: name.to_string(),
            temperature: *temp,
            condition: condition.to_string(),
            humidity: *humidity,
            wind_speed: *wind,
        })
        .collect();

    // Sort based on query parameter
    match params.sort.as_str() {
        "temp" | "temperature" => cities_data.sort_by_key(|c| c.temperature),
        "name" | "city" => cities_data.sort_by(|a, b| a.city.cmp(&b.city)),
        "humidity" => cities_data.sort_by_key(|c| c.humidity),
        "wind" => cities_data.sort_by_key(|c| c.wind_speed),
        _ => {} // default: no sorting
    }

    let total = cities_data.len();
    let avg_temp = cities_data.iter().map(|c| c.temperature).sum::<i32>() as f32 / total as f32;

    let hottest = cities_data
        .iter()
        .max_by_key(|c| c.temperature)
        .unwrap()
        .city
        .clone();

    let coldest = cities_data
        .iter()
        .min_by_key(|c| c.temperature)
        .unwrap()
        .city
        .clone();

    Json(StatsResponse {
        total_cities: total,
        average_temp: (avg_temp * 10.0).round() / 10.0,
        hottest_city: hottest,
        coldest_city: coldest,
        cities: cities_data,
    })
}

/// Get list of all available cities
async fn get_cities() -> impl IntoResponse {
    println!("🌍 Received cities list request");

    let weather_db = get_weather_database();
    let mut cities: Vec<String> = weather_db.keys().map(|k| k.to_string()).collect();
    cities.sort();

    Json(serde_json::json!({
        "count": cities.len(),
        "cities": cities
    }))
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_database_has_cities() {
        let db = get_weather_database();
        assert!(db.len() > 0, "Weather database should not be empty");
        assert!(db.contains_key("stockholm"), "Database should contain Stockholm");
        assert!(db.contains_key("gaza"), "Database should contain Gaza");
        assert!(db.contains_key("paris"), "Database should contain Paris");
    }

    #[test]
    fn test_weather_database_format() {
        let db = get_weather_database();

        // Check Stockholm has correct format
        if let Some((temp, condition, humidity, wind)) = db.get("stockholm") {
            assert!(*temp > -50 && *temp < 60, "Temperature should be in reasonable range");
            assert!(!condition.is_empty(), "Condition should not be empty");
            assert!(*humidity >= 0 && *humidity <= 100, "Humidity should be 0-100%");
            assert!(*wind >= 0, "Wind speed should be positive");
        }
    }

    #[test]
    fn test_all_cities_have_valid_data() {
        let db = get_weather_database();

        for (city, (temp, condition, humidity, wind)) in db.iter() {
            assert!(!city.is_empty(), "City name should not be empty");
            assert!(*temp > -60 && *temp < 60, "Temperature for {} should be reasonable", city);
            assert!(!condition.is_empty(), "Condition for {} should not be empty", city);
            assert!(*humidity >= 0 && *humidity <= 100, "Humidity for {} should be 0-100%", city);
            assert!(*wind >= 0 && *wind < 100, "Wind speed for {} should be reasonable", city);
        }
    }

    #[test]
    fn test_weather_database_count() {
        let db = get_weather_database();
        assert!(db.len() >= 40, "Database should have at least 40 cities");
    }
}
