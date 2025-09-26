//! CyREST client implementations for interacting with Cytoscape.
//!
//! This module provides both async and blocking client implementations
//! for communicating with the Cytoscape REST API. The clients handle
//! HTTP communication, request/response serialization, and error handling.

use crate::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_PORT: u16 = 1234;

/// Configuration for the CyREST client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the CyREST API.
    pub base_url: Url,
    /// Request timeout duration.
    pub timeout: Duration,
    /// Custom headers to include in requests.
    pub headers: HeaderMap,
}

#[cfg(test)]
mod tests {
    use super::*;
}