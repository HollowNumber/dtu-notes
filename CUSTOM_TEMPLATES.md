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

## Complete Example: Creating Your Own Custom Template

This section provides a complete, step-by-step walkthrough of creating and using a custom template.

### Step 1: Create Your Template Repository

Create a new GitHub repository with the following structure:

```
my-dtu-template/
├── README.md
└── template/
    ├── lib.typ
    └── template.typ
```

### Step 2: Write lib.typ - Template Functions

Create `template/lib.typ` with your custom styling:

```typst
#let my-custom-note(
  course: "",
  course-name: "",
  title: "",
  date: datetime.today(),
  author: "",
  semester: ""
) = {
  set document(title: title, author: author)
  set page(margin: 2.5cm, numbering: "1")

  // Custom header with your style
  align(center)[
    #rect(
      width: 100%,
      fill: rgb("#1f2937"),
      inset: 1em
    )[
      #text(size: 18pt, weight: "bold", fill: white)[
        #course - #course-name
      ]

      #v(0.5em)

      #text(size: 14pt, fill: white)[
        #title
      ]
    ]

    #v(0.5em)

    #text(size: 11pt)[
      #author | #semester | #date.display("[day]/[month]/[year]")
    ]
  ]

  v(1.5cm)

  // Set up your preferred styling
  set text(size: 11pt)
  set heading(numbering: "1.")

  show heading: it => [
    #v(1em)
    #block(
      fill: rgb("#f3f4f6"),
      width: 100%,
      inset: 0.5em,
      radius: 3pt
    )[
      #it
    ]
    #v(0.5em)
  ]
}

// Custom boxes for different content types
#let important(content) = block(
  fill: rgb("#fef3c7"),
  stroke: (left: 3pt + rgb("#f59e0b")),
  width: 100%,
  inset: 1em
)[
  #text(weight: "bold")[⚠️ Important]

  #content
]

#let example(content) = block(
  fill: rgb("#ecfdf5"),
  stroke: (left: 3pt + rgb("#10b981")),
  width: 100%,
  inset: 1em
)[
  #text(weight: "bold")[💡 Example]

  #content
]

#let note(content) = block(
  fill: rgb("#eff6ff"),
  stroke: (left: 3pt + rgb("#3b82f6")),
  width: 100%,
  inset: 1em
)[
  #text(weight: "bold")[📝 Note]

  #content
]
```

### Step 3: Write template.typ - Export Functions

Create `template/template.typ` to export your functions:

```typst
#import "lib.typ": my-custom-note, important, example, note
```

### Step 4: Add Your Template to Configuration

```bash
# Add your custom template repository
noter config add-template-repo my-style yourusername/my-dtu-template

# Check that it was added
noter config list-template-repos
```

### Step 5: Download and Test

```bash
# Download your template
noter template update

# Check status
noter template status

# Create a test note
noter note 02101
```

### Step 6: Using Your Custom Template

Your template will now be used when creating notes. The generated content will look like:

```typst
#import "@local/dtu-template:0.1.0": *

#show: my-custom-note.with(
  course: "02101",
  course-name: "Introduction to Programming",
  title: "Lecture - December 15, 2024",
  date: datetime.today(),
  author: "Your Name",
  semester: "2024 Fall"
)

= Key Concepts

#important[
  Key takeaways from today's lecture
]

= Mathematical Framework

= Examples

#example[
  Insert example here...
]

= Questions

#note[
  What questions do I have about this topic?
]
```

### Advanced: Multiple Templates for Different Purposes

You can manage multiple templates for different use cases:

```bash
# Add a minimal template for quick notes
noter config add-template-repo minimal john/minimal-dtu-template

# Add a formal template for assignments
noter config add-template-repo formal mary/formal-dtu-template

# List all templates (first enabled one will be used by default)
noter config list-template-repos

# Disable a template temporarily
noter config enable-template-repo formal false
```

### Template Priority Order

Templates are used in this order:

1. First enabled custom repository
2. Second enabled custom repository
3. Third enabled custom repository
4. ...
5. Official DTU template (if fallback is enabled)

To change the order, you need to remove and re-add repositories in your preferred order, or edit the config file directly at `noter config path`.

### Making Your Templates Available to Others

1. Create a GitHub release in your template repository
2. Tag it with a version (e.g., `v1.0.0`)
3. Share the repository name: `yourusername/my-dtu-template`
4. Others can add it with:
   ```bash
   noter config add-template-repo shared-template yourusername/my-dtu-template
   ```

### Troubleshooting Your Custom Template

If your template doesn't work as expected:

```bash
# Check what's installed and versions
noter template status

# Force reinstall all templates
noter template reinstall

# Test with a simple note
noter note test-course

# Check the generated file for errors
```

Common issues:
- **Template not found**: Make sure you've run `noter template update` after adding
- **Wrong template used**: Check order with `noter config list-template-repos`
- **Compilation errors**: Verify your Typst syntax in `lib.typ`
- **Functions not found**: Ensure `template.typ` exports all functions used

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
