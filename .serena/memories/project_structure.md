



# Project Structure

## Directory Layout:
```
arcdps_reporter/
├── Cargo.toml          # Project metadata and dependencies
├── Cargo.lock          # Generated lock file for reproducible builds
├── src/                # Source code directory
│   └── lib.rs          # Main plugin implementation
├── .gitignore          # Git ignore file
├── llms.txt            # Project context and memory
└── target/             # Build artifacts (ignored by git)
```

## Key Files:

### Cargo.toml
- Defines project name: "arcdps_exporter"
- Uses Rust 2024 edition
- Dependency: arcdps = "0.11.2"
- Configured as cdylib for DLL output

### src/lib.rs
- Contains the main plugin implementation
- Uses arcdps::arcdps_export! macro
- Implements combat_callback function
- Handles plugin initialization and combat events

### .gitignore
- Ignores target/ directory (build artifacts)
- Ignores DLL and SO files

### llms.txt
- Contains project documentation and development history
- Includes build instructions and technical details

## Build Artifacts:
- Output files go in target/release/
- Windows: arcdps_exporter.dll
- Linux: libarcdps_exporter.so

## Development Notes:
- The project follows standard Rust project structure
- Plugin signature should be changed from default 0x12345678
- Error handling and logging should be improved for production use



