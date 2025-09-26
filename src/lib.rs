//! # CyREST-rs
//!
//! A Rust client library for the Cytoscape CyREST API.
//!
//! This crate provides a comprehensive, type-safe interface to interact with
//! [Cytoscape](https://cytoscape.org/) via its REST API (CyREST). It enables
//! programmatic control over network visualization, analysis, and manipulation
//! tasks within Cytoscape.
//!
//! ## Features
//!
//! - **Async and Blocking APIs**: Choose between async (default) or blocking implementations
//! - **Type Safety**: Strongly typed request and response models
//! - **Comprehensive Coverage**: Support for all major CyREST endpoints
//! - **Error Handling**: Detailed error types for different failure scenarios
//! - **Builder Pattern**: Ergonomic API design with method chaining
//!
//! ## Quick Start..
//!
//! ### Async Example..
//!
//! ### Blocking Example..
//!
//! ## Modules
//!
//! - [`client`]: Main client implementations (async and blocking)
//! - [`error`]: Error types and error handling utilities
//! - [`models`]: Data structures for API requests and responses
//! - [`endpoints`]: API endpoint implementations organized by functionality
//!
//! ## Feature Flags
//!
//! - `async` (default): Enables the async client implementation using Tokio
//! - `blocking`: Enables the blocking client implementation
//!
//! ## Environment Setup
//!
//! Before using this library, ensure:
//! 1. Cytoscape 3.7.0 or higher is installed
//! 2. CyREST is enabled (included by default in Cytoscape 3.3+)
//! 3. Cytoscape is running and accessible on the expected port (default: 1234)
//!
//! ## Examples
//!
//! ### Creating a Network..
//!
//! ### Applying a Layout..
//!
//! ## Error Handling..
//!
//! ## Additional Resources
//!
//! - [CyREST Documentation](http://idekerlab.github.io/cyREST/)
//! - [Cytoscape Homepage](https://cytoscape.org/)
//! - [GitHub Repository](https://github.com/robmcdonald5/CyREST-rs)

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(unsafe_code)]

pub mod client;
pub mod error;

// Re-export main types for convenience
#[cfg(feature = "async")]
pub use client::CyRestClient;

#[cfg(feature = "blocking")]
pub use client::CyRestClientBlocking;

pub use error::{Error, Result};

// Module declarations that will be implemented
pub mod models {
    //! Data structures for CyREST API requests and responses.

    // Placeholder - to be implemented
}

pub mod endpoints {
    //! API endpoint implementations organized by functionality.

    // Placeholder - to be implemented
}