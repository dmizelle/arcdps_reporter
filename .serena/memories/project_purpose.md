# Project Purpose

This project is an extremely barebones arcdps plugin written in Rust using the arcdps crate. The plugin is designed to work with the arcdps (ArcDPS) combat data system for Guild Wars 2.

## Key Features:
- Written in Rust for safety and performance
- Uses the arcdps crate for Guild Wars 2 combat data access
- Configured as a cdylib to produce DLL files for Windows compatibility
- Implements basic combat event callback functionality
- Provides a foundation for more complex plugin development

## Current Functionality:
- Basic plugin initialization
- Combat event callback that prints messages when events occur
- Uses the arcdps::arcdps_export! macro for plugin definition
- Set up with standard Rust project structure

## Next Steps for Development:
- Add actual combat logging functionality
- Implement UI elements for configuration
- Add data analysis capabilities
- Integrate with external systems
- Implement proper error handling and logging
