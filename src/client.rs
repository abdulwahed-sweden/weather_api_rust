use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Serialize)]
struct WeatherRequest {
    cities: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    data: HashMap<String, WeatherData>,
}

#[derive(Debug, Deserialize)]
struct WeatherData {
    city: String,
    temperature: i32,
    condition: String,
    humidity: i32,
    wind_speed: i32,
}

#[derive(Debug, Deserialize)]
struct StatsResponse {
    total_cities: usize,
    average_temp: f32,
    hottest_city: String,
    coldest_city: String,
}

#[derive(Debug, Deserialize)]
struct CitiesResponse {
    count: usize,
    cities: Vec<String>,
}

/// Main entry point for the weather API client
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🦀 Rust Weather API Client v0.2.0");
    println!("====================================\n");

    let server_url = "http://localhost:3000";
    let client = reqwest::Client::new();

    // Check if server is running
    println!("🔍 Checking if server is ready...");
    match check_server_health(server_url).await {
        Ok(_) => println!("✅ Server is ready!\n"),
        Err(_) => {
            println!("❌ ERROR: Server is not running!");
            println!("\n💡 Please start the server first:");
            println!("   cargo run --bin server\n");
            std::process::exit(1);
        }
    }

    println!("{}", "=".repeat(60));
    println!("TEST 1: Get list of all available cities");
    println!("{}", "=".repeat(60));
    test_get_cities(&client, server_url).await?;

    println!("\n{}", "=".repeat(60));
    println!("TEST 2: Get weather for specific cities");
    println!("{}", "=".repeat(60));
    test_get_weather(&client, server_url).await?;

    println!("\n{}", "=".repeat(60));
    println!("TEST 3: Get weather statistics");
    println!("{}", "=".repeat(60));
    test_get_stats(&client, server_url).await?;

    println!("\n{}", "=".repeat(60));
    println!("TEST 4: Test error handling (empty request)");
    println!("{}", "=".repeat(60));
    test_error_handling(&client, server_url).await?;

    println!("\n🎉 All tests completed successfully!\n");

    Ok(())
}

/// Test 1: Get all available cities
async fn test_get_cities(client: &reqwest::Client, server_url: &str) -> Result<(), Box<dyn Error>> {
    let response = client
        .get(format!("{}/cities", server_url))
        .send()
        .await?;

    if response.status().is_success() {
        let cities_data: CitiesResponse = response.json().await?;
        println!("📍 Total cities available: {}", cities_data.count);
        println!("🌍 Cities: {}", cities_data.cities[..10].join(", "));
        println!("   ... and {} more", cities_data.count - 10);
    }

    Ok(())
}

/// Test 2: Get weather for specific cities
async fn test_get_weather(
    client: &reqwest::Client,
    server_url: &str,
) -> Result<(), Box<dyn Error>> {
    let cities = vec![
        "Stockholm".to_string(),
        "Gaza".to_string(),
        "Paris".to_string(),
        "Dubai".to_string(),
        "Tokyo".to_string(),
    ];

    println!("🌍 Requesting weather for: {}", cities.join(", "));

    let request = WeatherRequest { cities };
    let response = client
        .post(format!("{}/weather", server_url))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let weather_data: WeatherResponse = response.json().await?;

        println!("\n📊 Weather Details:\n");

        let mut cities: Vec<_> = weather_data.data.iter().collect();
        cities.sort_by_key(|&(city, _)| city);

        for (_city, data) in cities {
            println!("   🏙️  {}", data.city);
            println!("      🌡️  Temperature: {}°C", data.temperature);
            println!("      ☁️  Condition: {}", data.condition);
            println!("      💧 Humidity: {}%", data.humidity);
            println!("      💨 Wind Speed: {} km/h", data.wind_speed);
            println!();
        }
    }

    Ok(())
}

/// Test 3: Get weather statistics
async fn test_get_stats(client: &reqwest::Client, server_url: &str) -> Result<(), Box<dyn Error>> {
    let response = client
        .get(format!("{}/stats?sort=temp", server_url))
        .send()
        .await?;

    if response.status().is_success() {
        let stats: StatsResponse = response.json().await?;

        println!("📊 Weather Statistics:");
        println!("   📍 Total cities: {}", stats.total_cities);
        println!("   🌡️  Average temperature: {:.1}°C", stats.average_temp);
        println!("   🔥 Hottest city: {}", stats.hottest_city);
        println!("   ❄️  Coldest city: {}", stats.coldest_city);
    }

    Ok(())
}

/// Test 4: Test error handling
async fn test_error_handling(
    client: &reqwest::Client,
    server_url: &str,
) -> Result<(), Box<dyn Error>> {
    let empty_request = WeatherRequest { cities: vec![] };

    let response = client
        .post(format!("{}/weather", server_url))
        .json(&empty_request)
        .send()
        .await?;

    if response.status().is_client_error() {
        let error_text = response.text().await?;
        println!("✅ Error handling works correctly!");
        println!("   Response: {}", error_text);
    } else {
        println!("❌ Expected error but got success");
    }

    Ok(())
}

/// Check if the server is running and healthy
async fn check_server_health(server_url: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/", server_url))
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err("Server not healthy".into())
    }
}
