//! API endpoint implementations organized by functionality.
//!
//! This module contains implementations for different CyREST API
//! endpoint categories, providing a structured way to access
//! Cytoscape functionality.

pub mod networks;
pub mod commands;
pub mod collections;

// Re-export endpoint traits if needed in the future