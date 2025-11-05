# Template System Summary

## What We've Implemented

The DTU Notes CLI now supports a fully extensible template system that allows users to:

1. **Use custom GitHub repositories** for templates while keeping the official DTU template as a fallback
2. **Configure multiple template sources** with priority ordering
3. **Automatically download and cache** templates from GitHub releases
4. **Manage template repositories** through CLI commands
5. **Update templates** individually or all at once

## Key Features

### Configuration-Driven

- Templates are configured in the user's config file
- Support for multiple repositories with individual enable/disable
- Priority ordering and fallback behavior
- Auto-update settings

### GitHub Integration

- Fetches latest releases from any public GitHub repository
- Supports specific versions/tags or latest release
- Handles different repository structures (configurable template paths)
- Caches downloads for faster access

### User-Friendly Commands

- `noter config add-template-repo <name> <owner/repo>` - Add custom template
- `noter config list-template-repos` - List all configured templates
- `noter template status` - Check installed versions vs latest available
- `noter template update` - Update all templates to latest versions
- `noter template reinstall` - Force reinstall for troubleshooting

### Backward Compatibility

- Existing setups continue to work unchanged
- Official DTU template is used as fallback by default
- Legacy functions are preserved for existing integrations

## Configuration Structure

```toml
[templates]
use_official_fallback = true
enable_caching = true
auto_update = false
preference_order = ["my-template", "backup-template", "official"]

[[templates.custom_repositories]]
name = "my-template"
repository = "user/my-dtu-template"
version = "v2.1.0"  # Optional: specific version
template_path = "templates"  # Optional: subdirectory
enabled = true

[[templates.custom_repositories]]
name = "backup-template"
repository = "user/backup-template"
enabled = false
```

## Template Repository Structure

Templates should be structured as:

```
your-template-repo/
├── template/           # Default location (or specify with --template-path)
│   ├── lib.typ        # Your template functions
│   └── template.typ   # Export file
├── README.md
└── LICENSE
```

## Usage Examples

### Basic Usage

```bash
# Add a custom template
noter config add-template-repo my-style user/my-dtu-template

# Update templates
noter template update

# Create notes (uses your custom template)
noter note 02101
```

### Advanced Configuration

```bash
# Add template with specific version
noter config add-template-repo stable user/template --version v1.5.0

# Add template from subdirectory
noter config add-template-repo custom user/repo --template-path src/typst

# Manage repositories
noter config enable-template-repo old-template false
noter config remove-template-repo unused-template

# Check status
noter template status
```

## Implementation Details

### Core Components

1. **`GitHubTemplateFetcher`** - Handles downloading from multiple GitHub repositories
2. **`TemplateConfig`** - Configuration structure for template repositories
3. **`TemplateRepository`** - Individual repository configuration
4. **Template Commands** - CLI interface for template management
5. **Config Commands** - Repository management through configuration

### Automatic Template Resolution

The system tries repositories in this order:

1. First enabled custom repository
2. Second enabled custom repository
3. ...
4. Official DTU template (if `use_official_fallback = true`)

If a template download fails, it falls back to the next repository in the list.

### Caching and Performance

- Templates are cached locally after download
- Only downloads if not cached or when force-updating
- Checks GitHub API for latest versions without downloading
- Supports offline usage once templates are cached

## Benefits for Users

### 🎨 Complete Customization

- Users can create templates that match their personal/institutional style
- Support for different template types (notes, assignments, reports)
- Easy sharing of templates between users/teams

### Maintainable

- Templates are versioned and can be updated centrally
- Automatic update capabilities (optional)
- Fallback ensures reliability

### Extensible

- No changes needed to core CLI for new templates
- Templates can include custom functions, styling, and layouts
- Support for different Typst package structures

### Developer-Friendly

- Clear separation between template logic and CLI logic
- Configuration-driven approach
- Comprehensive error handling and user feedback

This implementation makes the DTU Notes CLI a truly extensible platform where users can adapt it to their specific needs while maintaining the benefits of the official DTU branding as a fallback option.
