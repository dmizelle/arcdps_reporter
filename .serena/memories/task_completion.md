




# Task Completion Guidelines

## When a Task is Completed:

1. **Code Changes**:
   - Ensure all code changes are made in src/lib.rs
   - Follow Rust coding conventions
   - Maintain proper error handling
   - Use Serena tools for structured edits when appropriate

2. **Testing**:
   - Run `cargo check` to verify compilation
   - Run `cargo build --release` to build the plugin
   - Test the plugin in the Guild Wars 2 environment
   - Use `execute_shell_command` for running tests

3. **Documentation**:
   - Update llms.txt with any new development information
   - Add comments to explain complex logic
   - Use `write_memory` to document important decisions
   - Update this memory if task completion steps change

4. **Version Control**:
   - Commit changes with descriptive messages
   - Push to the appropriate branch
   - Create pull requests for review if needed

5. **Build Artifacts**:
   - Copy the built DLL to the appropriate arcdps plugins directory
   - Ensure proper naming (arcdps_exporter.dll for Windows)

6. **Code Quality**:
   - Run `cargo fmt` if rustfmt is available
   - Consider running `cargo clippy` for linting
   - Use Serena tools for pattern-based refactoring

7. **Final Verification**:
   - Verify the plugin loads correctly in arcdps
   - Test combat events are handled properly
   - Check for any error messages or crashes

## Using Serena Tools for Task Completion:

### Exploration Phase:
- Use `list_dir` to understand project structure
- Use `read_file` to examine relevant files
- Use `get_symbols_overview` to understand code organization

### Implementation Phase:
- Use `find_symbol` to locate relevant code sections
- Use `replace_symbol_body` for function modifications
- Use `replace_regex` for pattern-based changes

### Documentation Phase:
- Use `write_memory` to document changes and decisions
- Use `read_memory` to reference existing documentation

### Testing Phase:
- Use `execute_shell_command` to run cargo commands
- Verify changes with appropriate test commands

## Common Issues to Check:
- Ensure callback function signatures match arcdps requirements
- Verify plugin signature is unique
- Check for proper memory safety in unsafe blocks
- Test on the target platform (Windows for arcdps)

## Deployment:
- For Windows users: Provide the .dll file
- For developers: Provide source code and build instructions
- Consider creating a release with pre-built binaries

## Best Practices:
- Use the right tool for the job (Serena vs shell commands)
- Document changes and decisions appropriately
- Test thoroughly before considering a task complete
- Keep the codebase clean and well-organized




