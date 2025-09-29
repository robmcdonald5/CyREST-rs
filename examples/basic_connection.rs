//! Basic example of connecting to Cytoscape with CyREST
//!
//! Run this example with:
//! cargo run --example basic_connection

use cyrest::SimpleClient;

#[tokio::main]
async fn main() {
    println!("CyREST Connection Example");
    println!("=========================\n");

    // Create a client
    let client = SimpleClient::new();
    println!("Attempting to connect to Cytoscape at localhost:1234...\n");

    // Try to ping Cytoscape
    match client.ping().await {
        Ok(info) => {
            println!("  Successfully connected to Cytoscape!");
            println!("  API Version: {}", info.api_version);
            println!("  All Apps Started: {}", info.all_apps_started);
            println!("  Number of Cores: {}", info.number_of_cores);
        }
        Err(e) => {
            println!("  Could not connect to Cytoscape");
            println!("  Error: {}", e);
            println!("\nTroubleshooting:");
            println!("1. Make sure Cytoscape is running");
            println!("2. Check that CyREST is enabled (it's on by default)");
            println!("3. Verify Cytoscape is using port 1234");
            return;
        }
    }

    // Try to get networks
    println!("\nFetching network list...");
    match client.get_networks().await {
        Ok(networks) => {
            println!("✓ Networks retrieved:");

            // Parse the JSON to make it prettier (optional)
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&networks) {
                println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
            } else {
                println!("{}", networks);
            }
        }
        Err(e) => {
            println!("  Failed to get networks: {}", e);
        }
    }

    println!("\n  Connection test complete!");
}