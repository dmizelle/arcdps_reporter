



# Suggested Commands for Development

## Essential Rust Commands:
- `cargo build --release`: Build optimized plugin
- `cargo check`: Quick compilation check
- `cargo test`: Run tests
- `cargo doc --open`: Open documentation
- `cargo clean`: Clean build artifacts

## Git Commands:
- `git status`: Check repository status
- `git add .`: Stage all changes
- `git commit -m "message"`: Commit changes
- `git push`: Push to remote
- `git pull`: Update from remote

## File Navigation:
- `ls -la`: List all files (including hidden)
- `cd src/`: Change to source directory
- `cat src/lib.rs`: View main source file

## System Commands:
- `pwd`: Show current directory
- `find . -name "*.rs"`: Find all Rust files
- `grep -r "function_name" .`: Search for function usage

## Plugin Development:
- `cp target/release/libarcdps_exporter.so path/to/arcdps/plugins/`: Copy plugin to arcdps directory
- `mv target/release/libarcdps_exporter.so target/release/arcdps_exporter.dll`: Rename for Windows

## Debugging:
- `RUST_BACKTRACE=1 cargo run`: Run with backtrace
- `cargo build --verbose`: Verbose build output

## Code Quality:
- `cargo fmt`: Format code (if rustfmt is installed)
- `cargo clippy`: Lint code (if clippy is installed)

## When to Use Serena Tools vs Shell Commands:

### Use Serena Tools When:
- You need structured code exploration and editing
- You want to maintain project documentation
- You need to find and replace code patterns safely
- You want to understand code symbols and structure

### Use Shell Commands When:
- You need to perform quick file operations
- You're running build and test commands
- You're working with git and version control
- You need to perform system-level operations

### Examples of Tool Selection:
- To explore project structure: `list_dir` (Serena) → `ls -la` (shell)
- To find a function: `find_symbol` (Serena) → `grep -r` (shell)
- To edit code: `replace_symbol_body` (Serena) → text editor (manual)
- To build: `execute_shell_command("cargo build")` (Serena) → `cargo build` (shell)

## Best Practices:
1. Use Serena tools for structured, documented changes
2. Use shell commands for quick operations and builds
3. Combine both approaches for efficient development
4. Document important decisions and changes using Serena memory tools



