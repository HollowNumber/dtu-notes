# DTU Notes CLI 🎓

A powerful command-line tool for managing lecture notes and assignments at the Technical University of Denmark (DTU). Built with Rust and designed to work seamlessly with Typst, Obsidian, and unofficial DTU templates.

## ✨ Features

- **📝 Note Creation**: Quickly create lecture notes and assignments with DTU branding
- **🔍 Smart Search**: Full-text search across all your notes
- **📊 Status Dashboard**: Overview of your courses and recent activity
- **🎯 Course Management**: Easy addition and organization of DTU courses
- **📦 Template System**: Unofficial DTU templates following the DTU Design Guide 2018
- **🔗 Obsidian Integration**: Generate course index files for knowledge management
- **⚡ Typst Compilation**: Built-in compilation and watch mode for PDF generation
- **🌐 Cross-Platform**: Works on Windows, macOS, and Linux

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (for building from source)
- [Typst](https://typst.app/) (for PDF compilation)
- A text editor (VS Code, Neovim, etc.)

### Installation

Clone the repository:

```bash
git clone <repository-url>
cd dtu-notes
```

Build and install:

```bash
cargo build --release
cargo install --path .
```

### Initial Setup

Initialize your note-taking environment:

```bash
noter setup
```

Configure your details:

```bash
noter config set-author "Your Full Name"
noter config set-editor code  # or nvim, vim, etc.
```

Check your setup:

```bash
noter status
```

## 📚 Usage

### Core Commands

Create a lecture note:

```bash
noter note 02101           # or: noter n 02101
```

Create an assignment:

```bash
noter assignment 02101 "Problem Set 1"    # or: noter a 02101 "Problem Set 1"
```

Open most recent note for a course:

```bash
noter open 02101           # or: noter o 02101
```

View recent notes for a course:

```bash
noter recent 02101         # or: noter r 02101
```

### Course Management

List your courses:

```bash
noter courses list
```

Add a new course:

```bash
noter courses add 02102 "Algorithms and Data Structures"
```

Remove a course:

```bash
noter courses remove 02102
```

Browse common DTU courses:

```bash
noter courses browse
```

### Compilation & Development

Compile a note to PDF:

```bash
noter compile notes/02101/lectures/2025-01-15-02101-lecture.typ
```

Watch for changes and auto-compile:

```bash
noter watch notes/02101/lectures/2025-01-15-02101-lecture.typ
```

Clean up compiled PDFs:

```bash
noter clean
```

### Search & Discovery

Search through all notes:

```bash
noter search "algorithm"   # or: noter s "algorithm"
```

View comprehensive status:

```bash
noter status
```

Show current semester info:

```bash
noter semester
```

### Configuration

Show current configuration:

```bash
noter config show
```

Update author name:

```bash
noter config set-author "Your Name"
```

Set preferred editor:

```bash
noter config set-editor nvim
```

Check configuration health:

```bash
noter config check
```

Reset to defaults:

```bash
noter config reset
```

### Obsidian Integration

Create course index for Obsidian:

```bash
noter index 02101          # or: noter i 02101
```

## 📁 Project Structure

After running `noter setup`, your project will have this structure:

```
your-notes/
├── notes/                    # Course notes
│   ├── 02101/
│   │   ├── lectures/         # Lecture notes (.typ files)
│   │   └── assignments/      # Assignment files (.typ files)
│   └── [other courses]/
├── obsidian-vault/          # Obsidian vault (optional)
│   ├── courses/             # Course index files
│   ├── weekly-reviews/      # Weekly review notes
│   └── concept-maps/        # Concept mapping notes
├── templates/               # DTU templates
│   └── dtu-template/        # DTU unofficial templates
├── README.md
└── .gitignore
```

## ⚙️ Configuration

Configuration is stored in your system's config directory:

- **Windows**: `%APPDATA%\dtu-notes\config.json`
- **macOS**: `~/Library/Application Support/dtu-notes/config.json`
- **Linux**: `~/.config/dtu-notes/config.json`

### Key Configuration Options

```json
{
  "author": "Your Name",
  "preferred_editor": "code",
  "template_version": "0.1.0",
  "note_preferences": {
    "auto_open": true,
    "include_date_in_title": true,
    "lecture_sections": [
      "Key Concepts",
      "Mathematical Framework",
      "Examples",
      "Important Points"
    ]
  },
  "paths": {
    "notes_dir": "notes",
    "obsidian_dir": "obsidian-vault",
    "templates_dir": "templates"
  }
}
```

## 🎨 Templates

DTU Notes uses unofficial DTU templates that follow the DTU Design Guide 2018. Templates include:

- **Lecture Notes**: Structured format with DTU branding
- **Assignments**: Problem set templates with proper formatting
- **Custom Sections**: Configurable sections for different note types

Templates are automatically installed to your Typst packages directory during setup.

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Development Setup

Clone and build:

```bash
git clone <repository-url>
cd dtu-notes
cargo build
```

Run tests:

```bash
cargo test
```

Run with debug logging:

```bash
RUST_LOG=debug cargo run -- status
```

### Code Style

- Follow Rust conventions (`cargo fmt`)
- Run Clippy for linting (`cargo clippy`)
- Add tests for new functionality
- Update documentation for new features

### Areas for Contribution

- 🐛 **Bug Fixes**: Report and fix issues
- ✨ **New Features**: Assignment due dates, better search, etc.
- 📚 **Documentation**: Improve docs and examples
- 🎨 **Templates**: Additional DTU course templates
- 🧪 **Testing**: More comprehensive test coverage

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Run `cargo test` and `cargo clippy`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to your branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📊 Status Dashboard

The `noter status` command provides a comprehensive overview:

- **System Status**: Directory and template validation
- **Recent Activity**: Latest notes and file statistics
- **Course Health**: Activity levels per course
- **Quick Suggestions**: Next recommended actions

## 🔍 Search Features

Powerful search capabilities across all your notes:

- **Case-insensitive search** by default
- **Configurable file types** (.typ, .md by default)
- **Context lines** around matches
- **Highlighted results** for easy scanning

## 📝 Tips & Best Practices

- **Consistent naming**: Course codes should be 5 digits (e.g., 02101)
- **Regular commits**: Use git to track changes to your notes
- **Backup important work**: Keep PDFs of important assignments
- **Use search**: Leverage full-text search to find information quickly
- **Status checks**: Run `noter status` regularly to stay organized

## 🐛 Troubleshooting

### Common Issues

**Templates not found**

```bash
noter status  # Check template status
noter setup   # Reinstall templates if needed
```

**Typst compilation fails**

```bash
typst --version  # Ensure Typst is installed
noter config check  # Validate configuration
```

**Files not opening**

```bash
noter config set-editor code  # Set your preferred editor
noter config show  # Check current settings
```

### Getting Help

- Check `noter --help` for command documentation
- Use `noter status` to diagnose setup issues
- Review configuration with `noter config show`
- Check the issue tracker for known problems

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Technical University of Denmark for the official branding guidelines
- The Typst team for the excellent typesetting system
- The Rust community for amazing tools and libraries
- Contributors who help improve this tool

---

**Happy note-taking at DTU! 🎓📚**