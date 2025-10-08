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
    data: HashMap<String, String>,
}

/// Main entry point for the weather API client
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🦀 Rust Weather API Client");
    println!("================================\n");

    // Server URL
    let server_url = "http://localhost:3000";

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

    // Prepare the request
    let cities = vec![
        "Stockholm".to_string(),
        "Gaza".to_string(),
        "Paris".to_string(),
    ];

    println!("🌍 Requesting weather for: {}", cities.join(", "));
    println!("{}", "-".repeat(50));

    let request = WeatherRequest { cities };

    // Send the request
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/weather", server_url))
        .json(&request)
        .send()
        .await?;

    // Parse the response
    if response.status().is_success() {
        let weather_data: WeatherResponse = response.json().await?;

        println!("\n📊 Weather Results:\n");

        // Sort cities alphabetically for consistent output
        let mut cities: Vec<_> = weather_data.data.iter().collect();
        cities.sort_by_key(|&(city, _)| city);

        for (_city, info) in cities {
            println!("   {}", info);
        }

        println!("\n✅ Request completed successfully!");
    } else {
        println!("❌ Error: Server returned status {}", response.status());
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
