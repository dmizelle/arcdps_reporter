


# Build and Development Commands

## Build Commands:
- `cargo build --release`: Build the plugin in release mode (optimized)
- `cargo build`: Build the plugin in debug mode
- `cargo check`: Quickly check for compilation errors without full build

## Testing Commands:
- `cargo test`: Run tests (currently no tests implemented)

## Documentation Commands:
- `cargo doc --open`: Generate and open documentation

## Cleaning:
- `cargo clean`: Remove build artifacts

## Plugin Output:
- Output DLL is in `target/release/arcdps_exporter.dll` (Windows)
- On Linux, output is `target/release/libarcdps_exporter.so`

## Development Workflow:
1. Make code changes in src/lib.rs
2. Run `cargo check` to verify compilation
3. Run `cargo build --release` to build the plugin
4. Copy the DLL to the appropriate arcdps plugins directory
5. Test in the Guild Wars 2 environment

## Common Issues:
- Callback signature mismatch: Ensure function signatures match arcdps requirements
- Macro issues: Use `arcdps::arcdps_export!` not just `arcdps_export!`
- DLL naming: Rename .so to .dll for Windows compatibility if needed


