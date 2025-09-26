//! Data structures for CyREST API requests and responses.
//!
//! This module contains all the data models used to interact with
//! the CyREST API, including network, node, edge, and visual style
//! representations.

pub mod network;
pub mod node;
pub mod edge;

// Re-export commonly used types
pub use network::{Network, NetworkBuilder};
pub use node::Node;
pub use edge::Edge;