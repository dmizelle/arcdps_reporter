
# Tech Stack

## Primary Language:
- Rust (2024 edition)

## Key Dependencies:
- arcdps = "0.11.2" - Rust crate providing arcdps plugin bindings

## Build System:
- Cargo (Rust's package manager and build system)

## Development Tools:
- cargo build --release: For building the plugin in release mode
- cargo check: For quickly checking compilation errors
- cargo doc --open: For opening documentation

## Output Format:
- cdylib (C-compatible dynamic library)
- Produces .so files on Linux, .dll files on Windows

## Development Environment:
- Standard Rust development environment
- Compatible with Windows for arcdps plugin usage
- Linux development supported with cross-compilation capabilities

