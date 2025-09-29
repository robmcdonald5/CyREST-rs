//! CyREST client implementations for interacting with Cytoscape.
//!
//! This module provides both async and blocking client implementations
//! for communicating with the Cytoscape REST API. The clients handle
//! HTTP communication, request/response serialization, and error handling.

use reqwest;
use serde::{Deserialize, Serialize};

/// Simple client for connecting to Cytoscape
pub struct SimpleClient {
    base_url: String,
    client: reqwest::Client,
}

/// Response from Cytoscape's /v1 endpoint with system info
#[derive(Debug, Deserialize, Serialize)]
pub struct CytoscapeInfo {
    /// The version of the CyREST API
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Whether all Cytoscape apps have finished starting
    #[serde(rename = "allAppsStarted")]
    pub all_apps_started: bool,
    /// Number of CPU cores available to Cytoscape
    #[serde(rename = "numberOfCores")]
    pub number_of_cores: u32,
}

impl SimpleClient {
    /// Create a new client connecting to localhost:1234 (default Cytoscape port)
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:1234".to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Create a client with a custom URL
    pub fn with_url(url: &str) -> Self {
        Self {
            base_url: url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Check if we can connect to Cytoscape
    pub async fn ping(&self) -> Result<CytoscapeInfo, String> {
        // Try to connect to the /v1 endpoint for system info (format! is required to build proper URL schema)
        let url = format!("{}/v1", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        // Check if we got a good response
        if !response.status().is_success() {
            return Err(format!("Bad response: {}", response.status()));
        }

        // Parse the JSON response
        response
            .json::<CytoscapeInfo>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get list of networks (returns raw JSON string for simplicity)
    pub async fn get_networks(&self) -> Result<String, String> {
        let url = format!("{}/v1/networks", self.base_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get networks: {}", e))?;

        response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))
    }
}