//! # Template Engine
//!
//! The template engine is responsible for generating Typst documents with dynamic
//! content based on course information, user preferences, and template configurations.
//!
//! ## Features
//!
//! - **Dynamic Version Detection**: Automatically detects installed template versions
//! - **Flexible Template Types**: Supports lectures, assignments, and custom templates
//! - **Context-Rich Generation**: Creates templates with comprehensive metadata
//! - **Section Management**: Configurable sections based on template type
//! - **Template Builder Pattern**: Fluent API for complex template creation
//!
//! ## Architecture
//!
//! The template engine follows a layered architecture:
//! - `TemplateEngine`: Core static methods for template generation
//! - `TemplateContext`: Rich data structure containing all template metadata
//! - `TemplateBuilder`: Fluent builder for complex template construction
//! - `TemplateType`: Enum defining supported template types
//!
//! ## Usage Examples
//!
//! ```rust
//! use dtu_notes::core::template_engine::*;
//! use dtu_notes::config::Config;
//!
//! let config = Config::default();
//!
//! // Generate lecture template
//! let lecture = TemplateEngine::generate_lecture_template("02101", &config, None)?;
//!
//! // Generate assignment template  
//! let assignment = TemplateEngine::generate_assignment_template(
//!     "02101",
//!     "Problem Set 1",
//!     &config
//! )?;
//!
//! // Use builder pattern for complex templates
//! let custom = TemplateBuilder::new("02101", &config)?
//!     .with_title("Custom Template")
//!     .with_type(TemplateType::Custom("research".to_string()))
//!     .with_sections(vec!["Methodology".to_string(), "Results".to_string()])
//!     .build()?;
//! ```

use anyhow::Result;
use chrono::Local;
use std::collections::HashMap;

use crate::config::Config;
use crate::core::github_template_fetcher::GitHubTemplateFetcher;
use crate::core::status_manager::StatusManager;
use crate::core::validation::Validator;

/// Rich context structure containing all metadata needed for template generation.
///
/// This structure encapsulates all the information required to generate a complete
/// Typst document, including course details, author information, date formatting,
/// and customizable sections.
///
/// ## Fields
///
/// - `course_id`: Course identifier (e.g., "02101")
/// - `course_name`: Full course name from configuration
/// - `title`: Document title (lecture name, assignment title, etc.)
/// - `author`: Author name from configuration
/// - `date`: Formatted date string for document metadata
/// - `semester`: Current semester string (e.g., "2025 Fall")
/// - `template_version`: Version of the template system to use
/// - `sections`: List of sections to include in the template
/// - `custom_fields`: Additional key-value pairs for specialized templates
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TemplateContext {
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub semester: String,
    pub template_version: String,
    pub sections: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

/// Enumeration of supported template types.
///
/// Each template type has specific formatting, sections, and metadata requirements:
///
/// - `Lecture`: Standard lecture notes with academic sections
/// - `Assignment`: Assignment templates with problem-solving sections  
/// - `Custom(String)`: User-defined templates with custom formatting
#[derive(Debug, Clone)]
pub enum TemplateType {
    Lecture,
    Assignment,
    Custom(String),
}

/// Main template engine providing static methods for template generation.
///
/// The `TemplateEngine` serves as the primary interface for creating Typst documents
/// with dynamic content. It handles template discovery, version management, and
/// content generation while ensuring consistency across all generated documents.
///
/// ## Key Features
///
/// - **Template Availability**: Ensures templates are downloaded and available
/// - **Dynamic Generation**: Creates context-aware templates with rich metadata
/// - **Version Management**: Automatically detects and uses correct template versions
/// - **File Naming**: Generates consistent, descriptive filenames
/// - **Validation**: Provides context validation and warning systems
pub struct TemplateEngine;

#[allow(dead_code)]
impl TemplateEngine {
    /// Ensure templates are available in the local environment.
    ///
    /// This method checks if templates are installed locally and downloads them
    /// from configured repositories if they are missing. It's typically called
    /// automatically before template generation to ensure the required templates
    /// are available.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration containing template repository information
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if templates are available or successfully downloaded,
    /// or an error if the download process fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Network connectivity issues prevent template download
    /// - Repository access is denied or repositories don't exist
    /// - File system operations fail during template installation
    /// - Template repositories contain invalid or corrupted templates
    pub fn ensure_templates_available(config: &Config) -> Result<()> {
        // Check if templates are already available
        let template_statuses = GitHubTemplateFetcher::check_template_status(config)?;

        // Check if we have any installed templates
        let has_templates = template_statuses
            .iter()
            .any(|(_, version)| version.is_some());

        if !has_templates {
            // No templates found, download from configured repositories
            let _download_results =
                GitHubTemplateFetcher::download_and_install_templates(config, false)?;
        }

        Ok(())
    }

    /// Generate a complete lecture note template with course-specific content.
    ///
    /// Creates a Typst document template optimized for lecture note-taking, including
    /// appropriate sections, formatting, and metadata. The template includes course
    /// information, author details, and configurable sections based on user preferences.
    ///
    /// # Arguments
    ///
    /// * `course_id` - Course identifier (e.g., "02101")
    /// * `config` - Application configuration containing course and user information
    /// * `custom_title` - Optional custom title; if None, generates default title with date
    ///
    /// # Returns
    ///
    /// Returns the complete Typst template as a string, ready to be written to a file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let template = TemplateEngine::generate_lecture_template("02101", &config, None)?;
    /// let custom = TemplateEngine::generate_lecture_template(
    ///     "02101",
    ///     &config,
    ///     Some("Advanced Topics")
    /// )?;
    /// ```
    pub fn generate_lecture_template(
        course_id: &str,
        config: &Config,
        custom_title: Option<&str>,
    ) -> Result<String> {
        let context = Self::build_lecture_context(course_id, config, custom_title)?;
        Self::render_template(&context, &TemplateType::Lecture)
    }

    /// Generate an assignment template with problem-solving sections.
    ///
    /// Creates a Typst document template specifically designed for assignments,
    /// including sections for problem statements, solutions, analysis, and conclusions.
    /// The template automatically configures due dates and assignment-specific formatting.
    ///
    /// # Arguments
    ///
    /// * `course_id` - Course identifier (e.g., "02101")
    /// * `assignment_title` - Title of the assignment (e.g., "Problem Set 1")
    /// * `config` - Application configuration
    ///
    /// # Returns
    ///
    /// Returns the complete assignment template as a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let assignment = TemplateEngine::generate_assignment_template(
    ///     "02101",
    ///     "Problem Set 1",
    ///     &config
    /// )?;
    /// ```
    pub fn generate_assignment_template(
        course_id: &str,
        assignment_title: &str,
        config: &Config,
    ) -> Result<String> {
        let context = Self::build_assignment_context(course_id, assignment_title, config)?;
        Self::render_template(&context, &TemplateType::Assignment)
    }

    /// Build lecture context
    fn build_lecture_context(
        course_id: &str,
        config: &Config,
        custom_title: Option<&str>,
    ) -> Result<TemplateContext> {
        let course_name = Self::resolve_course_name(course_id, config);
        let semester = StatusManager::get_current_semester(config);

        let title = if let Some(custom_title) = custom_title {
            custom_title.to_string()
        } else {
            let date = chrono::Local::now();
            if config.note_preferences.include_date_in_title {
                format!("Lecture - {}", date.format("%B %d, %Y"))
            } else {
                "Lecture Notes".to_string()
            }
        };

        Ok(TemplateContext {
            course_id: course_id.to_string(),
            course_name,
            title,
            author: config.author.clone(),
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            semester,
            template_version: config.template_version.clone(),
            sections: config.note_preferences.lecture_sections.clone(),
            custom_fields: HashMap::new(),
        })
    }

    /// Build assignment context
    fn build_assignment_context(
        course_id: &str,
        assignment_title: &str,
        config: &Config,
    ) -> Result<TemplateContext> {
        let course_name = Self::resolve_course_name(course_id, config);
        let semester = StatusManager::get_current_semester(config);

        Ok(TemplateContext {
            course_id: course_id.to_string(),
            course_name,
            title: assignment_title.to_string(),
            author: config.author.clone(),
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            semester,
            template_version: config.template_version.clone(),
            sections: config.note_preferences.assignment_sections.clone(),
            custom_fields: HashMap::new(),
        })
    }

    /// Render the template with the given context
    fn render_template(context: &TemplateContext, template_type: &TemplateType) -> Result<String> {
        let header = Self::generate_typst_header(context, template_type)?;
        let sections = Self::generate_sections(&context.sections, template_type)?;

        Ok(format!("{}\n{}", header, sections))
    }

    /// Generate the Typst document header
    fn generate_typst_header(
        context: &TemplateContext,
        template_type: &TemplateType,
    ) -> Result<String> {
        let template_name = match template_type {
            TemplateType::Lecture => "dtu-note",
            TemplateType::Assignment => "dtu-assignment",
            TemplateType::Custom(template) => template,
        };

        // For assignments, use due-date instead of date
        let date_param = match template_type {
            TemplateType::Assignment => "due-date",
            _ => "date",
        };

        // Use course name from config, or fall back to course_id if not available
        let course_name = if context.course_name.is_empty() {
            &context.course_id
        } else {
            &context.course_name
        };

        // Use the correct template import - try local package first, then fallback
        let import_statement = Self::determine_template_import(&context.template_version)?;

        Ok(format!(
            r#"{}

#show: {}.with(
  course: "{}",
  course-name: "{}",
  title: "{}",
  {}: datetime.today(),
  author: "{}",
  semester: "{}"
)"#,
            import_statement,
            template_name,
            context.course_id,
            course_name,
            context.title,
            date_param,
            context.author,
            context.semester,
        ))
    }

    /// Determine the correct template import statement
    fn determine_template_import(template_version: &str) -> Result<String> {
        // Get the actual installed template package name and version
        let (template_name, actual_version) =
            Self::get_installed_template_info().unwrap_or_else(|| {
                // Fallback to default if detection fails
                ("dtu-template".to_string(), template_version.to_string())
            });

        // Use the local package with the correct name and version
        let import_statement = format!("#import \"@local/{}:{}\":", template_name, actual_version);

        Ok(format!("{}*", import_statement))
    }

    /// Get the actual installed template package name and version
    fn get_installed_template_info() -> Option<(String, String)> {
        // Try to get a default config to check template status
        if let Ok(config) = crate::config::get_config() {
            if let Ok(template_statuses) = GitHubTemplateFetcher::check_template_status(&config) {
                // Look for any installed template and return the first one found
                for (name, version) in template_statuses {
                    if let Some(version) = version {
                        // The name from template status is the repository/package name
                        let package_name = Self::normalize_package_name(&name);
                        return Some((package_name, version));
                    }
                }
            }
        }

        // If we can't detect from status, try to read from template directories
        Self::read_template_info_from_files()
    }

    /// Normalize the package name for Typst imports
    fn normalize_package_name(name: &str) -> String {
        // Convert repository names to package names
        // e.g., "dtu_template" -> "dtu-template", "custom_repo" -> "custom-repo"
        name.replace('_', "-").to_lowercase()
    }

    /// Read template package name and version from installed template files
    fn read_template_info_from_files() -> Option<(String, String)> {
        // Try to get config to find template directories
        if let Ok(config) = crate::config::get_config() {
            // Check typst packages directory first
            let packages_dir = std::path::Path::new(&config.paths.typst_packages_dir);

            if let Some((name, version)) = Self::find_template_in_directory(&packages_dir) {
                return Some((name, version));
            }

            // Check templates directory as fallback
            let template_dir = std::path::Path::new(&config.paths.templates_dir);

            if let Some((name, version)) = Self::find_template_in_directory(&template_dir) {
                return Some((name, version));
            }
        }

        None
    }

    /// Find any template package in a directory
    fn find_template_in_directory(dir: &std::path::Path) -> Option<(String, String)> {
        if !dir.exists() {
            return None;
        }

        // Look for any subdirectory that contains a typst.toml
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(version) = Self::read_version_from_toml(&entry.path()) {
                        let package_name = entry.file_name().to_string_lossy().to_string();
                        return Some((package_name, version));
                    }
                }
            }
        }

        None
    }

    /// Read version from typst.toml file in a directory
    fn read_version_from_toml(dir: &std::path::Path) -> Option<String> {
        let toml_path = dir.join("typst.toml");

        if !toml_path.exists() {
            return None;
        }

        if let Ok(content) = std::fs::read_to_string(&toml_path) {
            // Simple regex-free parsing to find version = "x.y.z"
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("version") && line.contains("=") {
                    if let Some(equals_pos) = line.find('=') {
                        let version_part = line[equals_pos + 1..].trim();
                        // Remove quotes and whitespace
                        let version = version_part.trim_matches('"').trim_matches('\'').trim();
                        if !version.is_empty() {
                            return Some(version.to_string());
                        }
                    }
                }
            }
        }

        None
    }

    /// Generate section content based on template type
    fn generate_sections(sections: &[String], template_type: &TemplateType) -> Result<String> {
        let mut content = String::new();

        for section in sections {
            content.push_str(&format!("\n= {}\n", section));

            // Add type-specific content for certain sections
            match template_type {
                TemplateType::Lecture => {
                    content.push_str(&Self::generate_lecture_section_content(section));
                }
                TemplateType::Assignment => {
                    content.push_str(&Self::generate_assignment_section_content(section));
                }
                TemplateType::Custom(_) => {
                    content.push_str("\n\n");
                }
            }
        }

        Ok(content)
    }

    /// Generate content for lecture-specific sections
    fn generate_lecture_section_content(section: &str) -> String {
        match section {
            "Examples" => "\n#example[\n  Insert example here...\n]\n".to_string(),
            "Important Points" => {
                "\n#important[\n  Key takeaways from today's lecture\n]\n".to_string()
            }
            "Questions" => {
                "\n#question[\n  What questions do I have about this topic?\n]\n".to_string()
            }
            "Summary" => "\n#summary[\n  Brief summary of the main concepts\n]\n".to_string(),
            _ => "\n\n".to_string(),
        }
    }

    /// Generate content for assignment-specific sections
    fn generate_assignment_section_content(section: &str) -> String {
        match section {
            "Problem Statement" => "\n#problem[\n  State the problem clearly\n]\n".to_string(),
            "Solution" => "\n#solution[\n  Step-by-step solution\n]\n".to_string(),
            "Analysis" => "\n#analysis[\n  Analysis of the results\n]\n".to_string(),
            "Conclusion" => "\n#conclusion[\n  Final conclusions and insights\n]\n".to_string(),
            "Code" => "\n```python\n# Insert code here\n```\n".to_string(),
            "Calculations" => "\n$ \"Insert mathematical calculations here\" $\n".to_string(),
            _ => "\n\n".to_string(),
        }
    }

    /// Resolve course name from config
    fn resolve_course_name(course_id: &str, config: &Config) -> String {
        if let Some(course_name) = config.courses.get(course_id) {
            course_name.clone()
        } else {
            // Return empty string if course not found in config
            // The template will just use the course_id as fallback
            String::new()
        }
    }

    /// Generate filename for a template
    pub fn generate_filename(
        course_id: &str,
        template_type: &TemplateType,
        custom_title: Option<&str>,
    ) -> Result<String> {
        let date = Local::now().format("%Y-%m-%d").to_string();

        match template_type {
            TemplateType::Lecture => Ok(format!("{}-{}-lecture.typ", date, course_id)),
            TemplateType::Assignment => {
                if let Some(title) = custom_title {
                    let sanitized_title = Validator::sanitize_filename(title);
                    Ok(format!("{}-{}-{}.typ", date, course_id, sanitized_title))
                } else {
                    Ok(format!("{}-{}-assignment.typ", date, course_id))
                }
            }
            TemplateType::Custom(name) => {
                let sanitized_name = Validator::sanitize_filename(name);
                Ok(format!("{}-{}-{}.typ", date, course_id, sanitized_name))
            }
        }
    }

    /// Validate template context
    pub fn validate_context(context: &TemplateContext) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        if context.author.is_empty() {
            warnings.push("Author name is empty".to_string());
        }

        if context.course_name.is_empty() {
            warnings.push(format!("Course name not found for {}", context.course_id));
        }

        if context.template_version.is_empty() {
            warnings.push("Template version is not specified".to_string());
        }

        if context.sections.is_empty() {
            warnings.push("No sections configured for template".to_string());
        }

        Ok(warnings)
    }
}

/// Template builder for more complex template creation
pub struct TemplateBuilder {
    context: TemplateContext,
    template_type: TemplateType,
}

#[allow(dead_code)]
impl TemplateBuilder {
    pub fn new(course_id: &str, config: &Config) -> Result<Self> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let course_name = TemplateEngine::resolve_course_name(course_id, config);
        let semester = StatusManager::get_current_semester(config);

        Ok(Self {
            context: TemplateContext {
                course_id: course_id.to_string(),
                course_name,
                title: String::new(),
                author: config.author.clone(),
                date,
                semester,
                template_version: config.template_version.clone(),
                sections: Vec::new(),
                custom_fields: HashMap::new(),
            },
            template_type: TemplateType::Lecture,
        })
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.context.title = title.to_string();
        self
    }

    pub fn with_type(mut self, template_type: TemplateType) -> Self {
        self.template_type = template_type;
        self
    }

    pub fn with_sections(mut self, sections: Vec<String>) -> Self {
        self.context.sections = sections;
        self
    }

    pub fn add_custom_field(mut self, key: &str, value: &str) -> Self {
        self.context
            .custom_fields
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&self) -> Result<String> {
        TemplateEngine::render_template(&self.context, &self.template_type)
    }

    pub fn build_with_filename(&self) -> Result<(String, String)> {
        let content = self.build()?;
        let filename = TemplateEngine::generate_filename(
            &self.context.course_id,
            &self.template_type,
            Some(&self.context.title),
        )?;

        Ok((content, filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lecture_filename() {
        let filename =
            TemplateEngine::generate_filename("02101", &TemplateType::Lecture, None).unwrap();

        assert!(filename.contains("02101"));
        assert!(filename.contains("lecture"));
        assert!(filename.ends_with(".typ"));
    }

    #[test]
    fn test_generate_assignment_filename() {
        let filename = TemplateEngine::generate_filename(
            "02101",
            &TemplateType::Assignment,
            Some("Problem Set 1"),
        )
        .unwrap();

        assert!(filename.contains("02101"));
        assert!(filename.contains("problem-set-1"));
        assert!(filename.ends_with(".typ"));
    }

    #[test]
    fn test_sanitize_assignment_title() {
        let sanitized = Validator::sanitize_filename("Problem Set #1: Arrays & Pointers");
        assert_eq!(sanitized, "problem-set-1-arrays-pointers");
    }
}
