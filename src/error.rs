//! Error types and error handling utilities for the CyREST client.
//!
//! This module provides a comprehensive error handling system for all
//! operations in the CyREST client library. It uses the `thiserror`
//! crate for ergonomic error definitions and automatic `Display`
//! implementations.

use std::fmt;
use thiserror::Error;

/// Type alias for Results returned by this library.
pub type Result<T> = std::result::Result<T, Error>;

/// The main error type for the CyREST client library.
///
/// This enum represents all possible errors that can occur when
/// interacting with the CyREST API.
#[derive(Debug, Error)]
pub enum Error {
    /// Error taxonomy for HTTP-related errors
}

impl Error {
    /// Error helper functions
}

/// Error details for debugging purposes.
///
/// This struct provides additional context for errors when
/// verbose error reporting is needed.
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// The operation that was being performed when the error occurred.
    pub operation: String,
    /// Additional details about the error.
    pub details: Option<String>,
    /// The timestamp when the error occurred.
    pub timestamp: std::time::SystemTime,
}

impl ErrorContext {
    /// Create a new error context.
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            details: None,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Add details to the error context.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl fmt::Display for ErrorContext {
    /// Error context sugar formatting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation: {}", self.operation)?;
        if let Some(ref details) = self.details {
            write!(f, ", Details: {}", details)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}