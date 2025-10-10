# Config Migration System - Quick Reference

## Overview

DTU Notes now includes an automatic configuration migration system that prevents crashes when config format changes between versions. Your settings are preserved automatically when you update.

## For Users

### What Happens Automatically

✅ **Version check** on every config load  
✅ **Automatic migration** when version differs  
✅ **Settings preservation** - your courses, paths, and preferences are kept  
✅ **Backup creation** if migration fails (saved as `config.json.backup`)  

### Commands

```bash
# Check migration status
noter config migrate

# View current config version
noter config show

# Validate config
noter config check

# Show config location
noter config path
```

### Migration Messages

**Successful Migration:**
```
⚠️  Config format has changed. Migrating...
✓ Config migrated successfully!
```

**Recovery from Incompatible Config:**
```
⚠️  Failed to load config: missing field `new_field`
Creating backup and recovering values from old config...
Old config backed up to: ~/.config/dtu-notes/config.json.backup
✓ New config created with recovered values!
```

### If Something Goes Wrong

```bash
# 1. Check your backup
cat ~/.config/dtu-notes/config.json.backup

# 2. Restore important values
noter config set author "Your Name"
noter courses add 02101 "Programming"

# 3. If all else fails, reset (you'll lose settings!)
noter config cleanse --yes
```

## For Developers

### Quick Implementation Guide

**1. Adding New Optional Fields (Non-Breaking)**

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]  // ← Add this
pub struct YourStruct {
    pub existing_field: String,
    pub new_field: NewType,  // Will use Default if missing
}
```

**2. Making Breaking Changes**

```rust
// Step 1: Update version constant
impl Config {
    pub const CURRENT_VERSION: &'static str = "1.1.0";  // ← Increment
}

// Step 2: Add migration logic
fn migrate(mut config: Config) -> Result<Self> {
    match old_version {
        "1.0.0" => {
            // Perform migration (rename field, convert type, etc.)
            config.metadata.migration_notes = 
                "Migrated from 1.0.0 to 1.1.0: [describe changes]".to_string();
        }
        _ => {}
    }
    
    config.metadata.config_version = Self::CURRENT_VERSION.to_string();
    config.metadata.last_updated = chrono::Utc::now().to_rfc3339();
    
    Ok(config)
}

// Step 3: Update recovery logic if needed
fn recover_from_old_config(content: &str) -> Result<Self> {
    let old_value: serde_json::Value = serde_json::from_str(content)?;
    let mut new_config = Config::default();
    
    // Extract renamed/changed fields
    if let Some(old_field) = old_value.get("old_name") {
        // Convert to new format
    }
    
    Ok(new_config)
}
```

### Testing Checklist

- [ ] Add `#[serde(default)]` to all new structs
- [ ] Increment `CURRENT_VERSION` for breaking changes
- [ ] Add migration case in `migrate()` method
- [ ] Update recovery logic if field names changed
- [ ] Test with old config format
- [ ] Document changes in migration notes
- [ ] Update `MIGRATION_GUIDE.md` with version history

### Architecture

```
Config::load()
    ├── Config exists?
    │   ├── Yes → Try deserialize
    │   │   ├── Success → needs_migration()?
    │   │   │   ├── Yes → migrate() → save()
    │   │   │   └── No → return config
    │   │   └── Fail → backup + recover_from_old_config()
    │   └── No → create default + save()
    └── resolve_paths() → return
```

### Key Files

- `src/config.rs` - Config struct and migration logic
- `src/commands/config_cmd.rs` - CLI command handlers
- `MIGRATION_GUIDE.md` - Comprehensive documentation
- `MIGRATION_SUMMARY.md` - This file

### Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2024 | Initial versioned config with auto-migration |

### Best Practices

**DO:**
- ✅ Use `#[serde(default)]` on all config structs
- ✅ Test migrations with real old configs
- ✅ Document breaking changes in release notes
- ✅ Preserve user data during migration
- ✅ Increment version for breaking changes

**DON'T:**
- ❌ Remove old migration cases (users might skip versions)
- ❌ Change version without adding migration logic
- ❌ Make breaking changes without testing recovery
- ❌ Forget to update `CURRENT_VERSION` constant

## Implementation Details

### Metadata Struct

```rust
pub struct Metadata {
    pub config_version: String,      // Current: "1.0.0"
    pub created_at: String,           // ISO 8601 timestamp
    pub last_updated: String,         // ISO 8601 timestamp
    pub migration_notes: String,      // Human-readable migration log
}
```

### Three Migration Scenarios

1. **Compatible Migration** - Version changed but structure compatible
   - Uses `migrate()` to update version and apply specific migrations
   
2. **Incompatible Recovery** - Structure incompatible, deserialization fails
   - Uses `recover_from_old_config()` to extract values from JSON
   - Creates backup at `config.json.backup`
   
3. **No Migration** - Config version matches current
   - Loads normally, no changes needed

## See Also

- Full documentation: `MIGRATION_GUIDE.md`
- Config management: `noter config --help`
- Issue tracking: GitHub Issues