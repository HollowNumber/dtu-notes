# DTU Notes Documentation Index

Welcome to the comprehensive documentation for DTU Notes CLI! This index will help you find the information you need quickly.

## Documentation Structure

### For New Users

Start here if you're new to DTU Notes:

1. **[Main README](../README.md)** - Project overview, features, and quick start guide
2. **[Setup Guide](#getting-started)** - Initial configuration and installation
3. **[Basic Usage](#core-guides)** - Common commands and workflows

### For Template Creators

Everything you need to create and customize templates:

1. **[Custom Templates Guide](../CUSTOM_TEMPLATES.md)** - Complete guide with step-by-step example
2. **[Template Package Configuration](example.noter.config.toml)** - Example `.noter.config.toml` with all options
3. **[Template System Architecture](TEMPLATE_ARCHITECTURE.md)** - Implementation details and system design

### For Power Users

Advanced configuration and troubleshooting:

1. **[Configuration Management](CONFIG_MANAGEMENT.md)** - Deep dive into the config system
2. **[Configuration Quick Reference](CONFIG_QUICK_REFERENCE.md)** - Fast lookup for config options
3. **[Configuration Example](config-example.json)** - Complete example config file
4. **[Troubleshooting Guide](TROUBLESHOOTING.md)** - Common issues and solutions

### For Contributors & Developers

Technical documentation for those contributing to the project:

1. **[Development Guide](DEVELOPMENT.md)** - Setting up dev environment and contribution workflow
2. **[API Documentation](API.md)** - Library API for programmatic use
3. **[Migration Guide](MIGRATION_GUIDE.md)** - Version migration instructions
4. **[Migration Summary](MIGRATION_SUMMARY.md)** - Quick overview of migration changes

---

## Getting Started

### Installation

```bash
git clone https://github.com/HollowNumber/dtu-notes.git
cd dtu-notes
cargo build --release
```

### First-Time Setup

```bash
noter setup
```

This interactive wizard will:
- Create your workspace directory structure
- Download DTU templates
- Configure author information
- Set up Obsidian vault integration (optional)

### Essential Commands

```bash
noter note 02101                    # Create lecture note
noter assignment 02101 "HW 1"      # Create assignment
noter compile file.typ              # Compile to PDF
noter status                        # Check project health
```

For more commands, see the [Main README](../README.md#usage-guide).

---

## Core Guides

### Configuration

- **[Full Configuration Guide](CONFIG_MANAGEMENT.md)** - Comprehensive configuration documentation
  - Interactive configuration wizard
  - Dot notation for nested values
  - Configuration validation
  - Migration between versions

- **[Quick Reference](CONFIG_QUICK_REFERENCE.md)** - Fast lookup table
  - All configuration keys
  - Default values
  - Examples

- **[Example Config](config-example.json)** - Complete example configuration file

### Templates

- **[Custom Templates](../CUSTOM_TEMPLATES.md)** - Complete guide to custom templates
  - Adding template repositories
  - Managing multiple template sources
  - Template priority and fallbacks
  - Configuration options
  - Complete step-by-step example with code
  - Creating and publishing your own templates

- **[Template Configuration](example.noter.config.toml)** - Package configuration reference
  - Template definitions
  - Variants and course type mapping
  - Engine configuration
  - All available options with explanations

### Workflows

Common workflows and use cases:

#### Daily Note-Taking
```bash
noter note 02101                    # Today's lecture
noter recent 02101                  # View recent notes
noter open 02101                    # Open most recent
```

#### Assignment Management
```bash
noter assignment 02101 "Problem Set 1"
noter assignments health            # Track progress
noter compile path/to/assignment.typ
```

#### Course Organization
```bash
noter courses list                  # View all courses
noter courses add 02102 "Algorithms"
noter index 02101                   # Create Obsidian index
```

#### Search & Discovery
```bash
noter search "algorithm"            # Full-text search
noter status                        # Project overview
noter semester                      # Current semester info
```

---

## Reference Documentation

### Configuration Keys

See [CONFIG_QUICK_REFERENCE.md](CONFIG_QUICK_REFERENCE.md) for complete list of all configuration keys with descriptions and examples.

### API Reference

For programmatic use of DTU Notes as a library, see [API.md](API.md).

### Changelog

See [../CHANGELOG.md](../CHANGELOG.md) for version history and release notes.

---

## Development & Contributing

### Setting Up Development Environment

See [DEVELOPMENT.md](DEVELOPMENT.md) for:
- Development environment setup
- Code structure overview
- Building and testing
- Contribution guidelines

### Using the Justfile

Quick development tasks:

```bash
just build          # Build debug version
just test           # Run tests
just check          # Format + lint + test
just dev            # Quick dev cycle
just release        # Prepare release build
```

Run `just --list` to see all available commands.

### Architecture

- **[Template System](TEMPLATE_ARCHITECTURE.md)** - How template discovery and resolution works
- **[API Documentation](API.md)** - Library structure and public APIs
- **Codebase Structure:**
  - `src/commands/` - CLI command handlers
  - `src/core/` - Business logic
  - `src/ui/` - User interface components

---

## Getting Help

### Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for:
- Common errors and solutions
- Configuration issues
- Template problems
- Compilation errors
- Platform-specific issues

### Quick Diagnostics

```bash
noter status              # Check system health
noter config check        # Validate configuration
noter template status     # Check templates
```

### Support Channels

- **Issues**: [GitHub Issues](https://github.com/HollowNumber/dtu-notes/issues)
- **Discussions**: [GitHub Discussions](https://github.com/HollowNumber/dtu-notes/discussions)
- **Documentation**: You're reading it!

---

## Documentation Maintenance

### For Documentation Contributors

When adding new documentation:

1. Place user-facing guides in the root directory (e.g., `CUSTOM_TEMPLATES.md`)
2. Place technical/reference docs in `docs/` directory
3. Update this index file with links to new documentation
4. Keep examples up-to-date with current version
5. Add entries to [CHANGELOG.md](../CHANGELOG.md) for significant doc changes

### Documentation Style Guide

- Use clear, concise language
- Include code examples where applicable
- Add command output examples for CLI commands
- Keep examples tested and current

- Structure with clear headers and sections

---

## Quick Navigation

| I want to... | Go to... |
|--------------|----------|
| Get started quickly | [Main README](../README.md) |
| Configure DTU Notes | [Configuration Guide](CONFIG_MANAGEMENT.md) |
| Create custom templates | [Custom Templates](../CUSTOM_TEMPLATES.md) |
| Look up a config key | [Quick Reference](CONFIG_QUICK_REFERENCE.md) |
| Fix an issue | [Troubleshooting](TROUBLESHOOTING.md) |
| Contribute code | [Development Guide](DEVELOPMENT.md) |
| Use as a library | [API Documentation](API.md) |
| Migrate versions | [Migration Guide](MIGRATION_GUIDE.md) |
| See what's new | [Changelog](../CHANGELOG.md) |

---

**Last Updated**: 2025-01-15 (v0.6.0)

For questions or suggestions about documentation, please [open an issue](https://github.com/HollowNumber/dtu-notes/issues).
