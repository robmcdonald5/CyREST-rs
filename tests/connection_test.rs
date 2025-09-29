//! Integration tests for CyREST connection
//!
//! These tests verify that we can connect to a running Cytoscape instance.
//!
//! To run these tests:
//! 1. Start Cytoscape
//! 2. Run: cargo test --test connection_test

use cyrest::SimpleClient;

#[tokio::test]
async fn test_ping_cytoscape() {
    let client = SimpleClient::new();

    match client.ping().await {
        Ok(info) => {
            println!("  Connected to Cytoscape!");
            println!("  API Version: {}", info.api_version);
            println!("  All Apps Started: {}", info.all_apps_started);
            println!("  Number of Cores: {}", info.number_of_cores);

            // Basic assertions
            assert!(!info.api_version.is_empty());
            assert!(info.number_of_cores > 0);
        }
        Err(e) => {
            println!("  Could not connect to Cytoscape");
            println!("  Error: {}", e);
            println!("\n  Make sure Cytoscape is running on localhost:1234");
            panic!("Connection test failed");
        }
    }
}

#[tokio::test]
async fn test_get_networks() {
    let client = SimpleClient::new();

    // First check if we can connect
    if client.ping().await.is_err() {
        println!("Skipping network test - Cytoscape not running");
        return;
    }

    match client.get_networks().await {
        Ok(networks) => {
            println!("  Retrieved networks list");
            println!("  Response: {}", networks);

            // The response should at least be valid JSON (even if empty array)
            assert!(networks.starts_with('[') || networks.starts_with('{'));
        }
        Err(e) => {
            println!("  Could not get networks");
            println!("  Error: {}", e);
            panic!("Network retrieval failed");
        }
    }
}

#[tokio::test]
async fn test_custom_url() {
    // Test with a custom URL
    let client = SimpleClient::with_url("http://localhost:1234");

    // Just verify the client was created - actual connection test same as above
    match client.ping().await {
        Ok(_) => println!("  Custom URL client works"),
        Err(_) => println!("  Custom URL client failed (Cytoscape may not be running)"),
    }
}

/// Test connecting to wrong port (should fail gracefully)
#[tokio::test]
async fn test_connection_failure() {
    let client = SimpleClient::with_url("http://localhost:9999");

    match client.ping().await {
        Ok(_) => panic!("Should not connect to wrong port"),
        Err(e) => {
            println!("  Correctly failed to connect to wrong port");
            println!("  Error: {}", e);
            assert!(e.contains("Failed to connect") || e.contains("Connection refused"));
        }
    }
}