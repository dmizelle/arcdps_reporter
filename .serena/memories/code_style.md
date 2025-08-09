

# Code Style and Conventions

## Rust Code Style:
- Follow standard Rust formatting conventions
- Use 4-space indentation
- Use snake_case for function and variable names
- Use PascalCase for type names
- Use descriptive names for functions and variables

## Safety:
- Use Rust's ownership system for memory safety
- Mark unsafe blocks clearly when interfacing with C code
- Use proper error handling where appropriate

## Documentation:
- Use Rust doc comments (///) for public API documentation
- Use regular comments (//) for implementation notes
- Document the purpose of each major component

## Project Structure:
- Main code in src/lib.rs
- External dependencies managed via Cargo.toml
- Build artifacts in target/ directory (ignored by git)

## Plugin Development:
- Use arcdps::arcdps_export! macro for plugin definition
- Maintain exact function signatures required by arcdps
- Use unsafe extern \"C\" fn for callback functions
- Follow arcdps plugin development guidelines

## Example Code Style:
```rust
use arcdps::*;

unsafe extern \"C\" fn combat_callback(
    ev: Option<&CombatEvent>,
    _src: Option<&RawAgent>,
    _dst: Option<&RawAgent>,
    _skill_name: *mut i8,
    _id: u64,
    _revision: u64,
) {
    if let Some(event) = ev {
        println!("Combat event: {:?}", event.time);
    } else {
        println!("Plugin initialized");
    }
}

arcdps::arcdps_export! {
    name: \"Barebones Plugin\",
    sig: 0x12345678,
    raw_combat: combat_callback,
}
```

