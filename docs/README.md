# DTU Notes - Comprehensive Documentation

A powerful command-line interface for managing notes and assignments at DTU (Technical University of Denmark) with Typst and Obsidian integration.

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage Guide](#usage-guide)
- [Features](#features)
- [Architecture](#architecture)
- [Development](#development)
- [Troubleshooting](#troubleshooting)

## Quick Start

```bash
# Setup your workspace
noter setup

# Create a lecture note
noter note 02101

# Create an assignment
noter assignment 02101 "Problem Set 1"

# Compile to PDF
noter compile path/to/file.typ

# Check status
noter status
```

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Typst](https://typst.app/) CLI tool
- Git (for template management)

### From Source

```bash
git clone https://github.com/HollowNumber/dtu-notes.git
cd dtu-notes
cargo build --release
```

The binary will be available at `target/release/noter.exe` (Windows) or `target/release/noter` (Unix).

## Configuration

### Initial Setup

Run the setup wizard to configure your workspace:

```bash
noter setup
```

This will:

- Create directory structure
- Download DTU templates
- Configure author information
- Set up Obsidian vault (optional)
- Install Typst packages

### Configuration File

The configuration is stored in:

- Windows: `%APPDATA%/dtu-notes/config.json`
- macOS/Linux: `~/.config/dtu-notes/config.json`

### Manual Configuration

```bash
# Add courses
noter config add-course 02101 "Introduction to Programming"
noter config add-course 02105 "Algorithms and Data Structures"

# Configure paths
noter config set-path notes "D:/dtu/notes"
noter config set-path obsidian "D:/dtu/obsidian"

# Add custom template repositories
noter config add-template-repo custom-templates user/repo
```

## Usage Guide

### Creating Notes

#### Lecture Notes

```bash
# Create with default title
noter note 02101

# Create with custom title
noter template create 02101 "Custom Lecture Title"
```

#### Assignments

```bash
# Create assignment
noter assignment 02101 "Assignment 1"

# Alternative syntax
noter template create 02101 "Assignment Title" --type assignment
```

### Template Management

```bash
# Check template status
noter template status

# Update templates
noter template update

# Reinstall templates
noter template reinstall

# Add custom template repository
noter config add-template-repo name owner/repo
```

### Compilation

```bash
# Compile single file
noter compile file.typ

# Watch for changes (auto-compile)
noter watch file.typ

# Clean PDF files
noter clean
```

### Project Management

```bash
# Check system status
noter status

# Search files
noter search "query"

# List assignments with health analysis
noter assignments list
noter assignments health

# Check compilation status
noter check
```

## Features

### 🎓 Academic Focus

- **DTU Integration**: Pre-configured for DTU courses and structure
- **Course Management**: Automatic course detection and organization
- **Assignment Tracking**: Health monitoring and due date management

### 📝 Template System

- **Dynamic Templates**: Automatic template version detection
- **Customizable Sections**: Configurable lecture and assignment sections
- **Template Repositories**: Support for custom template sources
- **Version Management**: Automatic updates and compatibility checking

### 🔄 Workflow Integration

- **Obsidian Support**: Two-way sync with Obsidian vault
- **File Watching**: Auto-compilation on changes
- **Status Monitoring**: Comprehensive project health analysis
- **Smart Compilation**: Only compile when needed

### 🛠️ Developer Experience

- **Setup Wizard**: Guided first-time configuration
- **Status Dashboard**: Real-time project monitoring
- **Error Handling**: Comprehensive error reporting
- **Cross-Platform**: Windows, macOS, and Linux support

## Architecture

### Project Structure

```
dtu-notes/
├── src/
│   ├── commands/          # CLI command implementations
│   │   ├── assignments.rs # Assignment management
│   │   ├── config_cmd.rs  # Configuration commands
│   │   ├── courses.rs     # Course management
│   │   ├── info.rs        # Status and information
│   │   ├── notes.rs       # Note creation
│   │   ├── search.rs      # Search functionality
│   │   ├── setup.rs       # Setup wizard
│   │   ├── templates.rs   # Template management
│   │   └── typst.rs       # Compilation and watching
│   ├── core/              # Core business logic
│   │   ├── assignment_manager.rs
│   │   ├── course_management.rs
│   │   ├── directory_scanner.rs
│   │   ├── file_operations.rs
│   │   ├── github_template_fetcher.rs
│   │   ├── search_engine.rs
│   │   ├── setup_manager.rs
│   │   ├── status_manager.rs
│   │   ├── template_engine.rs
│   │   ├── typst_compiler.rs
│   │   └── validation.rs
│   ├── ui/                # User interface components
│   │   ├── formatters.rs  # Text formatting
│   │   ├── output.rs      # Output management
│   │   └── prompts.rs     # Interactive prompts
│   ├── config.rs          # Configuration management
│   ├── data.rs           # Static data (courses, etc.)
│   └── main.rs           # CLI entry point
├── docs/                 # Documentation
├── templates/           # Template storage
└── target/             # Build artifacts
```

### Key Components

#### Template Engine

- **Dynamic Version Detection**: Automatically detects installed template versions
- **Package Resolution**: Converts repository names to Typst package names
- **Context Building**: Creates rich template contexts with course information

#### Status Management

- **Health Scoring**: Analyzes course activity and file status
- **Activity Tracking**: Monitors recent changes and modifications
- **Comprehensive Reporting**: Provides actionable insights

#### File Operations

- **Safe Operations**: Atomic file operations with rollback
- **Path Management**: Cross-platform path handling
- **Backup System**: Automatic backup creation for important operations

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

### Code Organization

#### Commands Layer (`src/commands/`)

- Implements CLI command logic
- Handles user input validation
- Coordinates between UI and core modules

#### Core Layer (`src/core/`)

- Contains business logic
- Manages file operations
- Handles external integrations

#### UI Layer (`src/ui/`)

- Manages console output
- Handles user prompts
- Formats information display

### Adding New Features

1. **Add Command**: Extend `Commands` enum in `main.rs`
2. **Implement Logic**: Add handler in appropriate `commands/` module
3. **Add Core Logic**: Implement in relevant `core/` module
4. **Add Tests**: Create comprehensive test coverage
5. **Update Documentation**: Document new functionality

### Template Development

Templates use Typst with DTU-specific styling:

```typst
#import "@local/dtu-template:0.3.0":*

#show: dtu-note.with(
  course: "02101",
  course-name: "Introduction to Programming",
  title: "Lecture Notes",
  date: datetime.today(),
  author: "Your Name",
  semester: "2025 Fall"
)

= Introduction
Content goes here...
```

### Configuration Schema

```json
{
  "author": "Student Name",
  "template_version": "0.3.0",
  "semester_format": "YearSeason",
  "paths": {
    "notes_dir": "path/to/notes",
    "obsidian_vault_dir": "path/to/vault",
    "templates_dir": "path/to/templates",
    "typst_packages_dir": "path/to/packages"
  },
  "courses": {
    "02101": "Introduction to Programming"
  },
  "template_repositories": {
    "dtu_template": "HollowNumber/dtu-template"
  },
  "note_preferences": {
    "include_date_in_title": true,
    "lecture_sections": ["Key Concepts", "Examples"],
    "assignment_sections": ["Problem Statement", "Solution"]
  }
}
```

## Troubleshooting

### Common Issues

#### Template Compilation Errors

```bash
# Check template status
noter template status

# Reinstall templates
noter template reinstall

# Verify Typst installation
typst --version
```

#### File Not Found Errors

```bash
# Check configuration
noter config show

# Verify directory structure
noter status

# Re-run setup
noter setup --force
```

#### Permission Issues

```bash
# Check directory permissions
ls -la path/to/notes

# Fix ownership (Unix)
sudo chown -R $USER path/to/notes
```

### Debug Mode

Set `RUST_LOG=debug` for detailed logging:

```bash
RUST_LOG=debug noter note 02101
```

### Getting Help

```bash
# General help
noter --help

# Command-specific help
noter note --help
noter template --help
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests and documentation
5. Submit a pull request

### Code Style

- Follow Rust conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add comprehensive documentation
- Include error handling

## License

MIT License - see LICENSE file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/HollowNumber/dtu-notes/issues)
- **Discussions**: [GitHub Discussions](https://github.com/HollowNumber/dtu-notes/discussions)
- **Email**: Support available through GitHub

---

_Made with ❤️ for DTU students_
