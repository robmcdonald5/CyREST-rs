//! Diagnostic tool to see what Cytoscape is returning
//!
//! Run with: cargo run --example diagnostic

#[tokio::main]
async fn main() {
    println!("CyREST Diagnostic Tool");
    println!("======================\n");

    // Try to connect and see raw response
    let client = reqwest::Client::new();
    let url = "http://localhost:1234";

    println!("Attempting to connect to {}...\n", url);

    match client.get(url).send().await {
        Ok(response) => {
            println!("✓ Connection successful!");
            println!("  Status: {}", response.status());
            println!("  Headers: {:#?}\n", response.headers());

            // Get the raw text
            match response.text().await {
                Ok(text) => {
                    println!("Response body:");
                    println!("==============");
                    println!("{}", text);
                    println!("\nLength: {} characters", text.len());

                    // Try to parse as JSON
                    println!("\nAttempting JSON parse...");
                    match serde_json::from_str::<serde_json::Value>(&text) {
                        Ok(json) => {
                            println!("✓ Valid JSON!");
                            println!("{}", serde_json::to_string_pretty(&json).unwrap());
                        }
                        Err(e) => {
                            println!("✗ JSON parse failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to read response body: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Connection failed: {}", e);

            if e.is_connect() {
                println!("\nLikely causes:");
                println!("- Cytoscape is not running");
                println!("- Cytoscape is running on a different port");
                println!("- Firewall is blocking the connection");
            }
        }
    }

    // Also try the v1 endpoint
    println!("\n\nTrying /v1 endpoint...");
    println!("======================");

    match client.get("http://localhost:1234/v1").send().await {
        Ok(response) => {
            println!("Status: {}", response.status());
            if let Ok(text) = response.text().await {
                println!("Response: {}", text);
            }
        }
        Err(e) => {
            println!("Failed: {}", e);
        }
    }
}