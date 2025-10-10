# Configuration Migration Guide

## Overview

DTU Notes includes an automatic configuration migration system that prevents crashes when the config format changes between versions. This system ensures that your settings are preserved when you update to a new version.

## How It Works

### Automatic Migration

The migration system activates automatically when you run any command that loads the configuration. You don't need to do anything manually.

**What happens:**

1. **Version Check**: The system checks if your config version matches the current version
2. **Automatic Migration**: If versions differ, the config is automatically migrated
3. **Backup on Failure**: If migration fails, a backup is created at `config.json.backup`
4. **Value Recovery**: The system attempts to preserve all your custom settings
5. **Save Updated Config**: The migrated config is saved with the new version number

### Migration Output

When a migration occurs, you'll see output like this:

```
⚠️  Config format has changed. Migrating...
✓ Config migrated successfully!
```

Or in case of a major breaking change:

```
⚠️  Failed to load config: missing field `new_field`
Creating backup and recovering values from old config...
Old config backed up to: /home/user/.config/dtu-notes/config.json.backup
✓ New config created with recovered values!
```

## Manual Migration

If you want to manually check or trigger migration:

```bash
# Check migration status and trigger if needed
noter config migrate

# Check config validity
noter config check

# View current config version
noter config show
```

## Version Tracking

The config includes metadata that tracks version information:

```json
{
  "metadata": {
    "config_version": "1.0.0",
    "created_at": "2024-01-15T10:30:00Z",
    "last_updated": "2024-01-15T10:30:00Z",
    "migration_notes": "Migrated from 0.9.0 to 1.0.0"
  }
}
```

## What Gets Preserved

During migration, the system attempts to preserve:

- ✅ User name (`author`)
- ✅ Preferred editor
- ✅ All courses
- ✅ Template version
- ✅ Path configurations
- ✅ Note preferences
- ✅ Typst settings
- ✅ Search settings
- ✅ Template repositories
- ✅ Obsidian integration settings

## Handling Breaking Changes

### For Users

If a breaking change occurs:

1. **Backup is automatic**: Your old config is saved as `config.json.backup`
2. **Check the backup**: Review what settings were in your old config
3. **Manually adjust**: If needed, update the new config with any custom settings
4. **Use config commands**: Use `noter config set` to update specific values

Example:

```bash
# Check what your old author was from the backup
cat ~/.config/dtu-notes/config.json.backup | grep author

# Set it in the new config
noter config set author "Your Name"
```

### For Developers

When making changes to the config structure, follow these guidelines:

#### Adding New Fields (Non-Breaking)

When adding new optional fields, use `#[serde(default)]`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]  // This allows missing fields to use defaults
pub struct Config {
    pub existing_field: String,
    pub new_field: NewType,  // Will use Default::default() if missing
}
```

This allows old configs to load successfully with new fields using default values.

#### Making Breaking Changes

When making breaking changes (renaming fields, changing types, etc.):

1. **Increment the version** in `Config::CURRENT_VERSION`
2. **Add migration logic** in `Config::migrate()`:

```rust
fn migrate(mut config: Config) -> Result<Self> {
    let old_version = config.metadata.config_version.as_str();
    
    match old_version {
        "1.0.0" => {
            // Example: Rename field
            // config.new_name = config.old_name;
            config.metadata.migration_notes = 
                "Migrated from 1.0.0 to 1.1.0: Renamed field".to_string();
        }
        // ... other versions
        _ => {}
    }
    
    config.metadata.config_version = Self::CURRENT_VERSION.to_string();
    config.metadata.last_updated = chrono::Utc::now().to_rfc3339();
    
    Ok(config)
}
```

3. **Update recovery logic** in `Config::recover_from_old_config()` if needed:

```rust
fn recover_from_old_config(content: &str) -> Result<Self> {
    let old_value: serde_json::Value = serde_json::from_str(content)?;
    let mut new_config = Config::default();
    
    // Extract old field with new name
    if let Some(old_field) = old_value.get("old_name").and_then(|v| v.as_str()) {
        new_config.new_name = old_field.to_string();
    }
    
    // ... rest of recovery logic
    Ok(new_config)
}
```

4. **Test the migration**:
   - Create a config file with the old format
   - Run the application and verify migration works
   - Check that values are preserved

## Testing Migration

To test the migration system:

```bash
# 1. Find your config location
noter config path

# 2. Backup your current config
cp ~/.config/dtu-notes/config.json ~/config-backup.json

# 3. Create an old-format config for testing
# Edit the config and change the version to an old one, or remove a field

# 4. Run any command to trigger migration
noter status

# 5. Check migration worked
noter config show

# 6. Restore your original config if needed
cp ~/config-backup.json ~/.config/dtu-notes/config.json
```

## Troubleshooting

### Config Won't Load

If your config completely fails to load:

```bash
# Check what the error is
noter config check

# Try manual migration
noter config migrate

# If all else fails, reset (this will lose your settings!)
noter config cleanse --yes
```

### Lost Settings After Migration

If settings were lost during migration:

```bash
# 1. Check your backup file
cat ~/.config/dtu-notes/config.json.backup

# 2. Extract the values you need
# 3. Use config commands to restore them
noter config set author "Your Name"
noter courses add 02101 "Programming"
```

### Manual Config Edit Broke Things

If you manually edited the config and broke it:

```bash
# Validate what's wrong
noter config check

# Fix it with commands or editor
noter config edit

# Or start fresh
noter config reset
```

## Best Practices

### For Users

1. **Don't manually edit** the config file unless necessary
2. **Use CLI commands** (`noter config set`, etc.) when possible
3. **Keep backups** before major updates
4. **Check migration notes** after updates: `noter config show`

### For Developers

1. **Always use `#[serde(default)]`** on new structs
2. **Test with old configs** before releasing
3. **Document breaking changes** in release notes
4. **Increment version numbers** when making breaking changes
5. **Add migration logic** for each version transition

## Migration History

### Version 1.0.0
- Initial versioned config
- Added automatic migration system
- Added config metadata tracking

### Future Versions

When new versions are released with config changes, they will be documented here.

## See Also

- Configuration documentation: `noter config --help`
- Configuration management: `noter config show`
- Reset config: `noter config reset`
