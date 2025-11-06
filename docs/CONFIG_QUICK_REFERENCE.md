# Configuration Quick Reference

Quick reference for `noter config` commands.

## Most Common Commands

```bash
# Show all configuration
noter config show

# Interactive setup wizard
noter config interactive

# Get a value
noter config get <key>

# Set a value
noter config set <key> <value>

# List all available keys
noter config list-keys

# Open config file in editor
noter config edit
```

## Quick Examples

### Get/Set Values

```bash
# Author
noter config get author
noter config set author "Your Name"

# Editor
noter config get preferred_editor
noter config set preferred_editor "code"

# Paths
noter config get paths.notes_dir
noter config set paths.notes_dir "/path/to/notes"

# Booleans
noter config get templates.auto_update
noter config set templates.auto_update true

# Numbers
noter config get search.max_results
noter config set search.max_results 100
```

## Common Keys

| Key | Type | Description |
|-----|------|-------------|
| `author` | string | Your name |
| `preferred_editor` | string | Editor command |
| `template_version` | string | DTU template version |
| `paths.notes_dir` | string | Notes directory |
| `paths.obsidian_dir` | string | Obsidian vault |
| `paths.templates_dir` | string | Templates directory |
| `note_preferences.auto_open_file` | boolean | Auto-open after creation |
| `note_preferences.include_date_in_title` | boolean | Include dates in titles |
| `note_preferences.create_backups` | boolean | Create backups |
| `templates.auto_update` | boolean | Auto-update templates |
| `templates.enable_caching` | boolean | Cache templates |
| `templates.use_official_fallback` | boolean | Use official fallback |
| `search.max_results` | number | Max search results |
| `search.case_sensitive` | boolean | Case-sensitive search |
| `obsidian_integration.enabled` | boolean | Enable Obsidian |
| `obsidian_integration.create_course_index` | boolean | Create course index |

## Boolean Values

All of these work for boolean values:
- `true`, `false`
- `yes`, `no`
- `y`, `n`
- `1`, `0`

## Utility Commands

```bash
# Show config file location
noter config path

# Validate configuration
noter config check

# Reset to defaults
noter config reset

# Wipe everything
noter config cleanse
```

## Legacy Commands (Shortcuts)

```bash
# Set author (shortcut)
noter config set-author "Name"

# Set editor (shortcut)
noter config set-editor code

# Template repos
noter config add-template-repo <name> <owner/repo>
noter config remove-template-repo <name>
noter config enable-template-repo <name> true
noter config list-template-repos
noter config set-template-auto-update true
```

## Getting Help

```bash
# General help
noter config --help

# Command-specific help
noter config get --help
noter config set --help
```

## Config File Location

- **Windows**: `%APPDATA%\dtu-notes\config.json`
- **macOS**: `~/Library/Application Support/dtu-notes/config.json`
- **Linux**: `~/.config/dtu-notes/config.json`

---

For detailed documentation, see [CONFIG_MANAGEMENT.md](./CONFIG_MANAGEMENT.md)