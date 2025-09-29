# CyREST-rs

[![Crates.io](https://img.shields.io/crates/v/cyrest.svg)](https://crates.io/crates/cyrest)
[![Documentation](https://docs.rs/cyrest/badge.svg)](https://docs.rs/cyrest)
[![License](https://img.shields.io/crates/l/cyrest.svg)](https://github.com/yourusername/CyREST-rs#license)
[![Build Status](https://github.com/yourusername/CyREST-rs/workflows/CI/badge.svg)](https://github.com/yourusername/CyREST-rs/actions)

A Rust client library for the [Cytoscape](https://cytoscape.org/) CyREST API.

CyREST is a RESTful API that provides programmatic access to Cytoscape, enabling automation of network analysis, visualization, and manipulation tasks. This crate provides a type-safe, ergonomic Rust interface to the CyREST API.

## Features

- **Async and Blocking APIs** - Choose between async (default) or blocking client implementations
- **Type-Safe** - Strongly typed request and response models
- **Comprehensive Error Handling** - Detailed error types for different failure scenarios
- **Well Documented** - Extensive documentation with examples
- **Performance** - Efficient HTTP client with connection pooling
- **Tested** - Comprehensive test suite with mocked responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cyrest = "0.1.0"
```

### Feature Flags

- `async` (default): Enables async client using Tokio
- `blocking`: Enables blocking client implementation

For blocking client only:
```toml
[dependencies]
cyrest = { version = "0.1.0", default-features = false, features = ["blocking"] }
```

## Quick Start

### Async Example

## Core Functionality

### Network Operations
- Create, delete, and modify networks
- Import networks from various formats (CX, SIF, GraphML, etc.)
- Export networks to different formats

### Node and Edge Operations
- Add, remove, and update nodes and edges
- Query nodes and edges by properties
- Batch operations for efficiency

### Visual Style Management
- Create and modify visual styles
- Apply visual mappings
- Set default visual properties

### Layout Algorithms
- Apply various layout algorithms
- Configure layout parameters
- Batch layout operations

### Table Operations
- Access and modify node, edge, and network tables
- Import/export table data
- Perform table joins

### Command Interface
- Execute Cytoscape commands
- Access app-specific functionality
- Query available commands

## Examples

See the [examples](examples/) directory for more detailed usage examples

## Requirements

- Rust 1.70 or higher
- Cytoscape 3.7.0 or higher with CyREST enabled
- CyREST is included in Cytoscape 3.3+ by default

## Documentation

Full API documentation is available at [docs.rs/cyrest](https://docs.rs/cyrest).

For CyREST API documentation, see:
- [CyREST Documentation](http://idekerlab.github.io/cyREST/)
- Access Swagger docs in Cytoscape: Help → Automation → CyREST API

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Testing

Run the test suite:
```bash
cargo test
```

Run tests with coverage:
```bash
cargo tarpaulin
```

Run benchmarks:
```bash
cargo bench
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- The [Cytoscape](https://cytoscape.org/) team for creating an amazing network analysis platform
- The CyREST developers for providing the REST API
- The Rust community for excellent HTTP client libraries

## Related Projects

- [py4cytoscape](https://github.com/cytoscape/py4cytoscape) - Python client for CyREST
- [RCy3](https://github.com/cytoscape/RCy3) - R client for CyREST
- [cytoscape-automation](https://github.com/cytoscape/cytoscape-automation) - Collection of automation examples