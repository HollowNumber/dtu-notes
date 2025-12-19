 # Custom Templates Guide

The DTU Notes CLI supports custom template repositories, allowing you to use your own note templates while maintaining the official DTU template as a fallback.

## Quick Start

### 1. Add a Custom Template Repository

```bash
# Add your custom template repository
noter config add-template-repo my-template myusername/my-dtu-template

# Add with specific version
noter config add-template-repo stable-template myusername/template --version v2.1.0

# Add with custom template path (if templates are in a subdirectory)
noter config add-template-repo custom myusername/repo --template-path templates/dtu
```

### 2. List Your Template Repositories

```bash
noter config list-template-repos
```

### 3. Check Template Status

```bash
noter template status
```

### 4. Update Templates

```bash
# Update all templates to latest versions
noter template update

# Force reinstall (useful for fixing issues)
noter template reinstall
```

## Template Repository Structure

Your custom template repository should follow this structure:

```
your-template-repo/
├── template/                    # Default template location
│   ├── lib.typ                  # Your template functions
│   ├── template.typ             # Main template file
│   └── styles/                  # Optional styles directory
├── examples/                    # Example usage (optional)
├── README.md                    # Documentation
└── typst.toml                   # Typst package configuration (optional)
```

### Alternative Structure

If your templates are in a different location, specify the path:

```
your-repo/
├── src/typst/templates/         # Templates here
├── docs/
└── README.md
```

```bash
noter config add-template-repo myrepo user/repo --template-path src/typst/templates
```

## Template Configuration

### Enable/Disable Repositories

```bash
# Disable a repository temporarily
noter config enable-template-repo my-template false

# Re-enable it
noter config enable-template-repo my-template true

# Remove a repository completely
noter config remove-template-repo my-template
```

### Auto-Update Settings

```bash
# Enable automatic template updates (on startup)
noter config set-template-auto-update true

# Disable auto-updates
noter config set-template-auto-update false
```

## Priority and Fallback

Templates are used in this order:

1. **Custom repositories** (in the order they were added)
2. **Official DTU template** (as fallback, if enabled)

### Disabling Official Fallback

If you want to use only custom templates:

```toml
# Edit your config file: noter config path
[templates]
use_official_fallback = false
custom_repositories = [
    { name = "my-template", repository = "user/repo", enabled = true }
]
```

## Example: Creating Your Own Template

### 1. Create a GitHub Repository

Create a new repository with your template:

```
my-dtu-template/
└── template/
    ├── lib.typ
    └── template.typ
```

### 2. lib.typ - Template Functions

```typst
#let my-note(
  course: "",
  course-name: "",
  title: "",
  date: datetime.today(),
  author: "",
  semester: ""
) = {
  // Your custom template implementation
  set document(title: title, author: author)
  set page(margin: 2cm)

  align(center)[
    #text(size: 18pt, weight: "bold")[#course - #course-name]

    #text(size: 16pt)[#title]

    #text(size: 12pt)[#author | #semester | #date.display()]
  ]

  v(2cm)
}

#let my-assignment(
  course: "",
  course-name: "",
  title: "",
  due-date: datetime.today(),
  author: "",
  semester: ""
) = {
  // Your custom assignment template
  // Similar structure...
}
```

### 3. template.typ - Export Functions

```typst
#import "lib.typ": my-note, my-assignment
```

### 4. Add to Your Configuration

```bash
# Add your template
noter config add-template-repo my-style username/my-dtu-template

# Check it's working
noter template status

# Create a note using your template
noter note 02101
```

## Advanced Configuration

For more control, edit your config file directly:

```bash
# Find your config file
noter config path
```

```toml
[templates]
use_official_fallback = true
enable_caching = true
auto_update = false
preference_order = ["my-primary", "my-backup", "official"]

[[templates.custom_repositories]]
name = "my-primary"
repository = "myuser/primary-template"
version = "v2.0.0"
enabled = true

[[templates.custom_repositories]]
name = "my-backup"
repository = "myuser/backup-template"
branch = "main"
template_path = "typst/templates"
enabled = true
```

## Troubleshooting

### Template Not Found

```bash
# Check if repository exists and is accessible
noter template status

# Force reinstall
noter template reinstall
```

### Wrong Template Used

```bash
# Check repository order
noter config list-template-repos

# Disable unwanted repositories
noter config enable-template-repo unwanted-repo false
```

### Update Issues

```bash
# Clear cache and reinstall
noter template reinstall

# Check GitHub API rate limits
noter template status
```
