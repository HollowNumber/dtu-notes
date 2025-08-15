# Troubleshooting Guide

This guide covers common issues and their solutions when using DTU Notes.

## Common Issues

### Setup and Installation

#### "noter: command not found"

**Problem**: The `noter` command is not found after installation.

**Solutions**:

1. **Add to PATH**:

   ```bash
   # Add to your shell profile (.bashrc, .zshrc, etc.)
   export PATH="$PATH:/path/to/dtu-notes/target/release"
   ```

2. **Use full path**:

   ```bash
   /path/to/dtu-notes/target/release/noter status
   ```

3. **Install globally**:
   ```bash
   cargo install --path .
   ```

#### "Failed to initialize configuration"

**Problem**: Setup wizard fails during initialization.

**Solutions**:

1. **Check permissions**:

   ```bash
   # Ensure config directory is writable
   chmod 755 ~/.config/dtu-notes/
   ```

2. **Manual setup**:

   ```bash
   mkdir -p ~/.config/dtu-notes
   noter setup --force
   ```

3. **Clear corrupt configuration**:
   ```bash
   rm -rf ~/.config/dtu-notes
   noter setup
   ```

### Template Issues

#### "Template not found" or "Import failed"

**Problem**: Typst compilation fails with template import errors.

**Diagnosis**:

```bash
# Check template status
noter template status

# Check Typst packages
typst compile --help
```

**Solutions**:

1. **Reinstall templates**:

   ```bash
   noter template reinstall
   ```

2. **Update templates**:

   ```bash
   noter template update
   ```

3. **Check Typst installation**:

   ```bash
   typst --version
   # Should show version 0.11.0+
   ```

4. **Manual template verification**:
   ```bash
   ls -la ~/.config/dtu-notes/templates/
   ls -la ~/.local/share/typst/packages/
   ```

#### "Version mismatch" errors

**Problem**: Template version conflicts between different parts of the system.

**Solutions**:

1. **Clear template cache**:

   ```bash
   noter template reinstall --force
   ```

2. **Check configuration**:

   ```bash
   noter config show
   # Verify template_version matches installed templates
   ```

3. **Update configuration**:
   ```bash
   noter config set template-version 0.3.0
   ```

### Compilation Issues

#### "Typst compilation failed"

**Problem**: PDF generation fails with Typst errors.

**Diagnosis**:

```bash
# Test Typst directly
typst compile file.typ

# Check file syntax
typst check file.typ

# Debug compilation
noter compile file.typ --verbose
```

**Solutions**:

1. **Check file syntax**:

   - Ensure proper Typst syntax
   - Verify import statements
   - Check for missing closing brackets

2. **Template issues**:

   ```bash
   # Regenerate template
   noter template create 02101 "New Template"
   ```

3. **Path issues**:
   ```bash
   # Use absolute paths
   noter compile /full/path/to/file.typ
   ```

#### "File not found" during compilation

**Problem**: Typst can't find referenced files or templates.

**Solutions**:

1. **Check working directory**:

   ```bash
   cd /path/to/your/notes
   noter compile file.typ
   ```

2. **Use absolute paths**:

   ```bash
   noter compile "$PWD/file.typ"
   ```

3. **Verify file extensions**:
   ```bash
   # Ensure .typ extension
   mv file.txt file.typ
   ```

### Configuration Problems

#### "Invalid configuration format"

**Problem**: Configuration file is corrupted or has invalid JSON.

**Solutions**:

1. **Validate JSON**:

   ```bash
   cat ~/.config/dtu-notes/config.json | jq .
   ```

2. **Reset configuration**:

   ```bash
   mv ~/.config/dtu-notes/config.json ~/.config/dtu-notes/config.json.backup
   noter setup
   ```

3. **Use example configuration**:
   ```bash
   cp docs/config-example.json ~/.config/dtu-notes/config.json
   noter config set-author "Your Name"
   ```

#### "Course not found" errors

**Problem**: System can't find course information.

**Solutions**:

1. **Add course manually**:

   ```bash
   noter config add-course 02101 "Introduction to Programming"
   ```

2. **Check course list**:

   ```bash
   noter config list-courses
   ```

3. **Use course ID only**:
   ```bash
   # System will work with just course ID
   noter note 02101
   ```

### File System Issues

#### "Permission denied" errors

**Problem**: System can't create or modify files.

**Solutions**:

1. **Check directory permissions**:

   ```bash
   ls -la ~/notes/
   chmod 755 ~/notes/
   ```

2. **Fix ownership**:

   ```bash
   sudo chown -R $USER ~/notes/
   ```

3. **Use different directory**:
   ```bash
   noter config set-path notes "/path/to/writable/directory"
   ```

#### "Disk space" errors

**Problem**: Insufficient disk space for operations.

**Solutions**:

1. **Check available space**:

   ```bash
   df -h
   ```

2. **Clean up files**:

   ```bash
   noter clean  # Remove generated PDFs
   ```

3. **Change paths to different drive**:
   ```bash
   noter config set-path notes "/other/drive/notes"
   ```

### Network Issues

#### "Failed to download templates"

**Problem**: Template download fails due to network issues.

**Solutions**:

1. **Check connectivity**:

   ```bash
   curl -I https://github.com
   ```

2. **Retry with different network**:

   ```bash
   noter template update --retry 5
   ```

3. **Manual template download**:

   ```bash
   git clone https://github.com/HollowNumber/dtu-template.git
   cp -r dtu-template ~/.config/dtu-notes/templates/
   ```

4. **Use offline mode**:
   ```bash
   # Work with existing templates only
   noter template status --offline
   ```

### Performance Issues

#### "Slow compilation" or "High memory usage"

**Problem**: System becomes slow or uses too much memory.

**Solutions**:

1. **Limit concurrent operations**:

   ```bash
   # Edit config.json
   "max_concurrent_operations": 2
   ```

2. **Disable unnecessary features**:

   ```bash
   # In config.json
   "template_cache_enabled": false
   "health_monitoring.enabled": false
   ```

3. **Clean up temporary files**:
   ```bash
   noter clean --temp
   ```

## Debug Mode

For detailed troubleshooting information, enable debug mode:

```bash
# Enable debug logging
export RUST_LOG=debug
noter note 02101

# Enable backtraces
export RUST_BACKTRACE=1
noter compile file.typ

# Full backtrace
export RUST_BACKTRACE=full
noter template update
```

## System Information

When reporting bugs, include system information:

```bash
# System info
uname -a                    # Operating system
rustc --version            # Rust version
typst --version            # Typst version
noter --version            # DTU Notes version

# Configuration info
noter config show          # Current configuration
noter status               # System status
noter template status      # Template status

# File system info
ls -la ~/.config/dtu-notes/ # Config directory
df -h                       # Disk space
```

## Getting Help

### Command-line Help

```bash
noter --help                # General help
noter note --help          # Command-specific help
noter template --help      # Template commands help
```

### Verbose Output

```bash
# Enable verbose output for more information
noter --verbose note 02101
noter compile file.typ --verbose
```

### Community Support

- **GitHub Issues**: [Report bugs and request features](https://github.com/HollowNumber/dtu-notes/issues)
- **GitHub Discussions**: [Get help and discuss features](https://github.com/HollowNumber/dtu-notes/discussions)
- **Documentation**: [Read comprehensive docs](https://github.com/HollowNumber/dtu-notes/tree/main/docs)

### Bug Reports

When reporting bugs, please include:

1. **System Information**: OS, Rust version, Typst version
2. **DTU Notes Version**: `noter --version`
3. **Error Message**: Full error output
4. **Steps to Reproduce**: Minimal example that triggers the issue
5. **Configuration**: Relevant parts of your config (anonymized)
6. **Logs**: Debug output if applicable

### Feature Requests

For new features:

1. **Check existing issues** to avoid duplicates
2. **Describe the use case** and why it would be helpful
3. **Provide examples** of how the feature would work
4. **Consider implementation** complexity and maintenance burden

---

_If you can't find a solution here, don't hesitate to open an issue on GitHub!_
