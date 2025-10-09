# Configuration Management Guide

This guide covers all the ways to manage your `noter` configuration, from quick one-liners to interactive wizards.

## Table of Contents

- [Quick Start](#quick-start)
- [Configuration Commands](#configuration-commands)
- [Using Dot Notation](#using-dot-notation)
- [Configuration Structure](#configuration-structure)
- [Common Use Cases](#common-use-cases)
- [Tips & Best Practices](#tips--best-practices)

---

## Quick Start

### First Time Setup

The easiest way to configure `noter` is using the interactive wizard:

```bash
noter config interactive
```

This will walk you through the most common settings interactively.

### View Your Configuration

```bash
noter config show
```

### Quick Settings

```bash
# Set your name
noter config set author "Your Name"

# Set your preferred editor
noter config set preferred_editor "code"

# Enable template auto-updates
noter config set templates.auto_update true
```

---

## Configuration Commands

### Core Commands (New!)

#### `config show`
Display your entire configuration with smart formatting and colors.

```bash
noter config show
```

#### `config get <key>`
Get a specific configuration value using dot notation.

```bash
noter config get author
noter config get paths.notes_dir
noter config get templates.auto_update
```

#### `config set <key> <value>`
Set a configuration value using dot notation. The value type is automatically detected.

```bash
# Set strings
noter config set author "John Doe"

# Set booleans (accepts: true/false, yes/no, y/n, 1/0)
noter config set templates.auto_update true
noter config set note_preferences.auto_open_file yes

# Set numbers
noter config set search.max_results 100
```

#### `config list-keys`
List all available configuration keys you can get/set.

```bash
noter config list-keys
```

#### `config edit`
Open the configuration file in your preferred editor.

```bash
noter config edit
```

The config file will be validated after you close the editor.

#### `config interactive`
Launch an interactive wizard to configure common settings.

```bash
noter config interactive
```

### Legacy Commands (Still Supported)

These commands are shortcuts for common operations:

```bash
# Set author (shortcut for: config set author "Name")
noter config set-author "Your Name"

# Set editor (shortcut for: config set preferred_editor "editor")
noter config set-editor code

# Template repository management
noter config add-template-repo <name> <owner/repo>
noter config remove-template-repo <name>
noter config enable-template-repo <name> true
noter config list-template-repos
noter config set-template-auto-update true
```

### Utility Commands

```bash
# Show config file path
noter config path

# Validate configuration
noter config check

# Reset to defaults
noter config reset

# Completely wipe and start fresh
noter config cleanse
```

---

## Using Dot Notation

Dot notation allows you to access nested configuration values easily.

### Syntax

```
<section>.<subsection>.<field>
```

### Examples

| Dot Notation Path | Description |
|-------------------|-------------|
| `author` | Your name |
| `preferred_editor` | Your editor command |
| `template_version` | DTU template version |
| `paths.notes_dir` | Notes directory |
| `paths.obsidian_dir` | Obsidian vault directory |
| `paths.templates_dir` | Templates directory |
| `note_preferences.auto_open_file` | Auto-open after creation |
| `note_preferences.include_date_in_title` | Include dates in titles |
| `templates.auto_update` | Auto-update templates |
| `templates.enable_caching` | Enable template caching |
| `search.max_results` | Max search results |
| `search.case_sensitive` | Case-sensitive search |
| `obsidian_integration.enabled` | Enable Obsidian integration |

### Get All Available Keys

To see the complete list of keys you can use:

```bash
noter config list-keys
```

---

## Configuration Structure

The configuration is stored as JSON at:
- **Windows**: `%APPDATA%\dtu-notes\config.json`
- **macOS**: `~/Library/Application Support/dtu-notes/config.json`
- **Linux**: `~/.config/dtu-notes/config.json`

### Main Sections

```
├── author                    # Your name for templates
├── preferred_editor          # Your text editor
├── template_version          # DTU template version
├── semester_format           # How to format semester names
├── paths                     # Directory paths
│   ├── notes_dir
│   ├── obsidian_dir
│   ├── templates_dir
│   └── typst_packages_dir
├── note_preferences          # Note creation preferences
│   ├── auto_open_file
│   ├── auto_open_dir
│   ├── include_date_in_title
│   ├── create_backups
│   ├── lecture_sections
│   └── assignment_sections
├── templates                 # Template management
│   ├── custom_repositories
│   ├── use_official_fallback
│   ├── enable_caching
│   ├── auto_update
│   └── preference_order
├── search                    # Search preferences
│   ├── max_results
│   ├── case_sensitive
│   ├── context_lines
│   └── file_extensions
├── obsidian_integration      # Obsidian settings
│   ├── enabled
│   ├── create_course_index
│   ├── link_format
│   └── tag_format
└── typst                     # Typst compilation
    ├── compile_args
    ├── watch_args
    └── output_dir
```

---

## Common Use Cases

### Change Your Name

```bash
noter config set author "Your Full Name"
```

### Set Your Editor

```bash
# VS Code
noter config set preferred_editor "code"

# Neovim
noter config set preferred_editor "nvim"

# Vim
noter config set preferred_editor "vim"

# Emacs
noter config set preferred_editor "emacs"
```

### Change Notes Directory

```bash
noter config set paths.notes_dir "/path/to/your/notes"
```

### Enable/Disable Auto-Open

```bash
# Enable
noter config set note_preferences.auto_open_file true

# Disable
noter config set note_preferences.auto_open_file false
```

### Configure Search

```bash
# Increase max results
noter config set search.max_results 100

# Enable case-sensitive search
noter config set search.case_sensitive true
```

### Template Management

```bash
# Enable auto-update
noter config set templates.auto_update true

# Enable caching
noter config set templates.enable_caching true

# Add custom template repository
noter config add-template-repo mytemplate owner/repo

# List template repositories
noter config list-template-repos
```

### Obsidian Integration

```bash
# Enable Obsidian integration
noter config set obsidian_integration.enabled true

# Enable course index creation
noter config set obsidian_integration.create_course_index true

# Change link format
noter config set obsidian_integration.link_format "wiki"
```

---

## Tips & Best Practices

### 1. Use Tab Completion

If your shell supports it, tab completion works with the `config` subcommands.

### 2. Check Before You Set

Always check the current value before changing it:

```bash
noter config get templates.auto_update
noter config set templates.auto_update false
noter config get templates.auto_update  # Verify
```

### 3. Use Interactive Mode for Initial Setup

The interactive wizard is perfect for first-time setup:

```bash
noter config interactive
```

### 4. Back Up Your Config

Before making major changes:

```bash
# Find config location
noter config path

# Copy it manually, or use the edit command carefully
noter config edit
```

### 5. Validate After Manual Edits

If you edit the config file directly:

```bash
noter config check
```

### 6. Use `list-keys` to Discover Options

Don't know what you can configure?

```bash
noter config list-keys
```

### 7. Boolean Values Are Flexible

When setting boolean values, you can use:
- `true`, `false`
- `yes`, `no`
- `y`, `n`
- `1`, `0`

```bash
noter config set templates.auto_update yes
noter config set templates.auto_update y
noter config set templates.auto_update true
noter config set templates.auto_update 1
# All of these work!
```

### 8. Edit Complex Values Directly

For arrays and complex objects, use the `edit` command:

```bash
noter config edit
```

Then edit the JSON directly in your editor.

### 9. Reset If Something Goes Wrong

If your config gets messed up:

```bash
# Reset to defaults (keeps courses)
noter config reset

# Complete wipe and start fresh
noter config cleanse
```

### 10. Use Absolute Paths

For directory paths, use absolute paths to avoid confusion:

```bash
# Good
noter config set paths.notes_dir "/home/user/notes"

# Avoid relative paths
# noter config set paths.notes_dir "../notes"
```

---

## Troubleshooting

### Configuration Not Loading

```bash
# Check config file exists
noter config path

# Validate configuration
noter config check

# Reset if corrupted
noter config reset
```

### Can't Find a Key

```bash
# List all available keys
noter config list-keys

# Or view the entire config
noter config show
```

### Editor Not Opening

```bash
# Set your editor explicitly
noter config set preferred_editor "code"

# Or set the EDITOR environment variable
export EDITOR=vim
noter config edit
```

### Boolean Not Working

Make sure you're using a valid boolean value:

```bash
# ✅ Good
noter config set templates.auto_update true

# ❌ Bad
noter config set templates.auto_update True  # Case matters!
noter config set templates.auto_update on    # Not recognized
```

---

## Advanced: Manual Editing

You can also edit the config file directly:

```bash
# Open in your editor
noter config edit

# Or find the path and open it yourself
noter config path
```

The configuration is stored in JSON format:

```json
{
  "author": "Your Name",
  "preferred_editor": "code",
  "paths": {
    "notes_dir": "/path/to/notes",
    "obsidian_dir": "/path/to/obsidian"
  },
  "templates": {
    "auto_update": false,
    "enable_caching": true
  }
}
```

**Remember to validate after manual edits:**

```bash
noter config check
```

---

## See Also

- [Main README](../README.md)
- [Template Management](./TEMPLATES.md) (if exists)
- [Course Management](./COURSES.md) (if exists)

---

## Examples Cheat Sheet

```bash
# View all config
noter config show

# Interactive setup
noter config interactive

# Get specific value
noter config get author

# Set specific value
noter config set author "John Doe"

# List all keys
noter config list-keys

# Open in editor
noter config edit

# Validate config
noter config check

# Show file path
noter config path

# Reset to defaults
noter config reset
```

---
