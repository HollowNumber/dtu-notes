# Changelog

All notable changes to DTU Notes will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1] - 2025-08-21

### Bug fixes:

- Fixed issue where it wouldn't select newest version of a template
- Fixed issue where course mappings werent being implemented


## [0.5.0] - 2025-08-21

### Added
- **Enhanced Template Engine**: Complete rewrite of the template processing system
  - Advanced template discovery with multi-source support (local, remote, custom repositories)
  - Dynamic template variant selection based on course codes and types
  - Rich template context system with comprehensive metadata support
  - Template validation and error reporting with actionable suggestions
- **Template Configuration System**: New `.noter.config.toml` format for template packages
  - Template metadata and version tracking
  - Course type mapping for automatic variant selection
  - Engine capability declarations and feature flags
  - Processing hooks and variable transformation support
- **Fluent Template Builder API**: New builder pattern for template creation
  - Method chaining for template customization
  - Validation integration with detailed error reporting
  - Support for custom sections and template references
- **Template Discovery Engine**: Intelligent template detection and loading
  - Multi-source template resolution (builtin, local, remote, custom)
  - Template accessibility checking and health monitoring
  - Package information extraction and version compatibility
- **Advanced Template Variants**: Course-specific template specialization
  - Math-specific assignments with proof and analysis sections
  - Programming assignments with implementation and testing workflows
  - Physics lab reports with error analysis and theoretical background
  - Automatic variant selection based on DTU course code patterns

### Enhanced
- **File Operations**: Expanded file management capabilities
  - Enhanced directory structure creation with course-specific layouts
  - Improved file existence checking and conflict resolution
  - Better error handling for file system operations
  - Cross-platform path resolution and backup management
- **Configuration Management**: Robust config handling and migration
  - Field completion migration system (automatically adds missing fields)
  - Configuration validation with helpful error messages
  - Backward compatibility preservation for existing configs
  - Better default value handling for new installations
- **Template Content Generation**: More sophisticated document creation
  - Dynamic section generation based on template definitions
  - Variable substitution with transformation support
  - Import statement generation with version detection
  - Typst function call generation with proper parameter passing

### Fixed
- **Version Inconsistencies**: Resolved mismatches between README and actual version
  - Updated README badge to reflect current version (0.5.0)
  - Synchronized documentation versions across all files
  - Fixed template version references in examples and guides
- **Dead Code Elimination**: Cleaned up unused imports and functions
  - Removed unused `TemplateContext` import from notes.rs
  - Added proper `#[allow(dead_code)]` attributes for intentional unused code
  - Reduced compiler warning noise from 67+ warnings to manageable levels
- **Template System Reliability**: Improved template generation robustness
  - Better error handling in template discovery and loading
  - Fixed template variant selection edge cases
  - Improved template file path resolution across platforms
  - Enhanced template configuration parsing with better defaults

### Changed
- **Template Architecture**: Restructured template system for better maintainability
  - Separated template discovery from template generation
  - Modular template context building with builder pattern
  - Clear separation between template configuration and runtime context
  - Improved abstraction layers between CLI commands and template engine
- **Command Structure**: Refined command-line interface for better usability
  - Cleaner separation of concerns between commands and file operations
  - More consistent error messages and status reporting
  - Better integration between template system and CLI layer
- **Configuration Schema**: Evolved config structure for enhanced functionality
  - Added `typst` configuration section for compilation settings
  - Enhanced `search` configuration with more granular options
  - Improved `templates` section with repository management
  - Better default values and field organization

### Technical Improvements
- **Code Organization**: Better module structure and responsibility separation
  - Template system split into focused modules (engine, discovery, context, validation)
  - Cleaner boundaries between business logic and file operations
  - Reduced coupling between template system and CLI commands
- **Error Handling**: More comprehensive error reporting with context
  - Template validation errors include specific field references
  - File operation errors provide actionable suggestions
  - Configuration errors explain how to fix issues
- **Performance**: Optimized template generation and file operations
  - Template caching for faster repeated operations
  - Reduced memory allocations in template processing
  - Better directory scanning with file type filtering

### Breaking Changes
- **Template Configuration Format**: New `.noter.config.toml` structure
  - Migration: Old template repositories will be automatically converted
  - Impact: Custom template packages may need updates to new format
  - Benefit: Much more flexible and extensible template system

### Migration Guide
- **From 0.4.x to 0.5.0**:
  1. **Manual Migration**: This is a breaking change. You must run `noter config reset` to fix the configuration issues.
  2. **New Features**: Access enhanced template variants with `--variant` flag

## [0.4.0] - 2025-08-15

### Added
- **Development Tools**: Added optional `dev-tools` feature for development workflows
    - New `dev` command with subcommands for generating sample data
    - `dev simulate` - Generate high-yield simulation data
    - `dev generate` - Generate sample data with custom parameters
    - `dev clean` - Clean all generated development data
- **Conditional Compilation**: Development tools are only compiled when `--features dev-tools` is specified
- **Template Management**: Enhanced template engine with better error handling and validation

### Changed
- **Dependencies**: Updated to modern crate versions
    - `ureq` 3.0.12 - Updated HTTP client with new API
    - `zip` 4.3.0 - Replaced deprecated `zip-extract` with standard `zip` crate
    - `rand` 0.9.2 - Updated random number generation with proper feature flags
- **Archive Handling**: Improved ZIP extraction using `extract_unwrapped_root_dir` for cleaner directory structures
- **HTTP Responses**: Updated to new `ureq` 3.x API with `body_mut()` and `read_to_string()` methods
- **Random Generation**: Fixed `StdRng` usage with proper `SeedableRng` trait imports and `seed_from_u64`

### Fixed
- **Compilation Errors**: Resolved trait bound issues with HTTP response body reading
- **Method Resolution**: Fixed deprecated method calls in `ureq` and `zip` crates
- **Feature Gates**: Properly gated development dependencies behind `dev-tools` feature
- **Template Extraction**: Improved reliability of template downloading and installation

### Technical Improvements
- **Build Configuration**: Optional dependencies now properly excluded from production builds
- **Error Handling**: Enhanced error context throughout the codebase
- **Code Organization**: Better separation between development and production features
- **Documentation**: Improved inline documentation for development tools

### Breaking Changes
- Development tools are no longer available by default - must use `--features dev-tools`
- Some internal APIs changed due to dependency updates (affects library usage only)

### Migration Guide
- To use development tools, install with: `cargo install --path . --features dev-tools`
- For development: `cargo run --features dev-tools -- dev simulate`
- Production builds remain unchanged: `cargo install --path .`



## [0.3.0] - 2025-08-15

### Added

- **Dynamic Template Version Detection**: Templates now automatically detect and use the correct installed version instead of hardcoded versions
- **Template Package Name Resolution**: Support for converting repository names to Typst package names (e.g., `dtu_template` → `dtu-template`)
- **Advanced Assignment Management**: Added assignment health monitoring with visual status indicators (🟢🟡🟠🔴)
- **Comprehensive Status Dashboard**: Enhanced status command with detailed system monitoring
- **Compilation Status Monitoring**: Added `noter check` command for detailed file compilation status analysis
- **Assignment Health Analysis**: Track assignment activity and provide actionable recommendations
- **Setup Wizard Integration**: Comprehensive first-time setup experience
- **Multi-layer Template Detection**: Fallback system for template version detection using multiple sources
- **Comprehensive Documentation**: Added detailed API documentation, development guides, and usage examples
- **Warning-Free Codebase**: Eliminated all compiler warnings with strategic `#[allow(dead_code)]` attributes

### Enhanced

- **Template Engine**: Complete rewrite of version detection system for better reliability
- **Error Handling**: Improved error messages with better context and actionable suggestions
- **Code Organization**: Better separation of concerns with dedicated modules for each functionality
- **Performance**: Optimized file operations and template generation
- **User Experience**: More intuitive command structure and helpful feedback messages

### Fixed

- **Template Compilation Failures**: Resolved issues where templates used wrong version imports
- **File Path Resolution**: Fixed cross-platform path handling issues
- **Configuration Validation**: Better validation of configuration files and user inputs
- **Template Repository Management**: Improved handling of custom template repositories

### Technical Improvements

- **Architecture**: Layered architecture with clear separation between CLI, business logic, and I/O
- **Testing**: Added comprehensive unit and integration tests
- **Documentation**: Extensive inline documentation and external guides
- **Code Quality**: Applied consistent formatting and linting across the codebase
- **Build System**: Optimized build configuration for both development and release

### Dependencies

- Updated `clap` to 4.5.42 for improved CLI parsing
- Updated `chrono` to 0.4.41 with serde features
- Added comprehensive error handling with `anyhow` 1.0.98
- Improved JSON handling with `serde_json` 1.0.142

## [0.2.0] - 2025-08-01

### Added

- **Course Management**: Automatic course detection and organization
- **Obsidian Integration**: Two-way sync with Obsidian vaults
- **Template Repositories**: Support for custom template sources from GitHub
- **File Watching**: Auto-compilation with `noter watch` command
- **Search Functionality**: Search across notes and assignments
- **Configuration System**: JSON-based configuration with user preferences

### Enhanced

- **Template System**: More flexible template generation with custom sections
- **CLI Interface**: Improved command structure with aliases and help text
- **File Operations**: Safer file operations with backup and rollback

## [0.1.0] - 2025-07-15

### Added

- **Initial Release**: Basic note and assignment creation
- **Typst Integration**: PDF compilation support
- **Basic Templates**: Lecture and assignment templates
- **Simple Configuration**: Basic configuration management
- **CLI Framework**: Command-line interface using clap

### Features

- Create lecture notes with `noter note`
- Create assignments with `noter assignment`
- Compile Typst files with `noter compile`
- Basic status checking with `noter status`

## Development Milestones

### Upcoming Features (0.4.0)

- [ ] **Advanced UI Components**: Enhanced tables, progress bars, and interactive prompts
- [ ] **Template Validation**: Context-aware template validation with suggestions
- [ ] **Advanced Search**: Full-text search with filtering and sorting
- [ ] **Export Options**: Multiple export formats (HTML, Markdown, etc.)
- [ ] **Collaboration Features**: Shared templates and collaborative editing
- [ ] **Plugin System**: Extensible plugin architecture
- [ ] **Web Interface**: Optional web-based interface for remote access

### Long-term Goals (1.0.0)

- [ ] **University Integration**: Support for multiple universities and course systems
- [ ] **Advanced Analytics**: Detailed usage analytics and productivity insights
- [ ] **Cloud Sync**: Cloud-based synchronization and backup
- [ ] **Mobile App**: Companion mobile application
- [ ] **AI Integration**: AI-powered note organization and content suggestions

## Breaking Changes

### 0.3.0

- Template import statements now use dynamic version detection
- Configuration file format extended with new template repository fields
- Some internal APIs changed for better modularity

### 0.2.0

- Configuration file format changed to JSON
- Command aliases modified for consistency
- Template directory structure reorganized

## Migration Guide

### From 0.2.x to 0.3.0

1. **Template Updates**: Run `noter template update` to refresh templates
2. **Configuration**: No changes needed - configuration is backward compatible
3. **Templates**: Existing templates will automatically use correct version detection

### From 0.1.x to 0.2.0

1. **Configuration**: Migrate from TOML to JSON format using `noter setup`
2. **Templates**: Re-download templates using `noter template reinstall`
3. **Commands**: Update any scripts using old command names

## Contributors

- **Mikkel M.H Pedersen** - Initial development and architecture
- **GitHub Community** - Bug reports, feature requests, and feedback

## Acknowledgments

- **DTU (Technical University of Denmark)** - Institutional support and requirements
- **Typst Team** - Excellent typesetting system
- **Rust Community** - Amazing ecosystem and tools
- **Open Source Contributors** - Various libraries and inspirations

---

For more detailed information about specific changes, see the [commit history](https://github.com/HollowNumber/dtu-notes/commits/main) on GitHub.
