



# Available Tools and Their Usage

## File and Directory Operations:
1. **list_dir** - List files and directories
   - Use when: You need to explore the project structure
   - Example: `list_dir("src", recursive=true)`

2. **read_file** - Read file contents
   - Use when: You need to examine specific files
   - Example: `read_file("src/lib.rs")`

3. **create_text_file** - Create new files
   - Use when: You need to add new files to the project
   - Example: `create_text_file("src/new_module.rs", "pub fn example() {}")`

4. **find_file** - Find files by name pattern
   - Use when: You need to locate specific files
   - Example: `find_file("*.rs", "src")`

## Code Analysis and Editing:
5. **get_symbols_overview** - Get high-level symbol information
   - Use when: You need to understand the structure of a file
   - Example: `get_symbols_overview("src/lib.rs")`

6. **find_symbol** - Find specific symbols (functions, classes, etc.)
   - Use when: You need to locate specific code elements
   - Example: `find_symbol("combat_callback")`

7. **replace_regex** - Replace text using regular expressions
   - Use when: You need to make pattern-based edits
   - Example: `replace_regex("src/lib.rs", "old_pattern", "new_pattern")`

8. **replace_symbol_body** - Replace the body of a symbol
   - Use when: You need to update function implementations
   - Example: `replace_symbol_body("combat_callback", "new implementation")`

## Search and Pattern Matching:
9. **search_for_pattern** - Search for text patterns in code
   - Use when: You need to find specific code patterns
   - Example: `search_for_pattern("CombatEvent", "src")`

## Memory and Documentation:
10. **write_memory** - Create documentation memory files
    - Use when: You need to document important information
    - Example: `write_memory("new_feature.md", "Documentation content")`

11. **read_memory** - Read existing memory files
    - Use when: You need to reference documented information
    - Example: `read_memory("project_purpose.md")`

## System and Development:
12. **execute_shell_command** - Run shell commands
    - Use when: You need to perform system operations
    - Example: `execute_shell_command("cargo build --release")`

## When to Use Each Tool:
- **Exploration**: Start with `list_dir`, `read_file`, `get_symbols_overview`
- **Searching**: Use `find_symbol`, `search_for_pattern`, `find_file`
- **Editing**: Use `replace_regex`, `replace_symbol_body`, `create_text_file`
- **Documentation**: Use `write_memory`, `read_memory`
- **Build/Testing**: Use `execute_shell_command` for cargo commands

## Best Practices:
1. Always explore before editing - understand the context
2. Use specific tools for specific tasks (e.g., `find_symbol` for code navigation)
3. Document important decisions and project information
4. Test changes with appropriate cargo commands
5. Keep the project structure organized and clean


