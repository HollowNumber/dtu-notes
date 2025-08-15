use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// User's name for templates
    pub author: String,

    /// Preferred text editor
    pub preferred_editor: Option<String>,

    /// DTU template version to use
    pub template_version: String,

    /// Semester format preference
    pub semester_format: SemesterFormat,

    /// Default note structure preferences
    pub note_preferences: NotePreferences,

    /// Paths configuration
    pub paths: PathConfig,

    /// Typst compilation settings
    pub typst: TypstConfig,

    /// Search preferences
    pub search: SearchConfig,

    /// User's DTU courses
    pub courses: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotePreferences {
    /// Whether to automatically open files after creation
    pub auto_open: bool,

    /// Include date in lecture note titles
    pub include_date_in_title: bool,

    /// Default sections for lecture notes
    pub lecture_sections: Vec<String>,

    /// Default sections for assignments
    pub assignment_sections: Vec<String>,

    /// Whether to create backup of existing files
    pub create_backups: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathConfig {
    /// Base directory for notes
    pub notes_dir: String,

    /// Obsidian vault directory
    pub obsidian_dir: String,

    /// Templates directory
    pub templates_dir: String,

    /// Typst packages directory
    pub typst_packages_dir: String,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            notes_dir: "notes".to_string(),
            obsidian_dir: "obsidian-vault".to_string(),
            templates_dir: "templates".to_string(),
            // Use data_local_dir which maps to the right location on each OS:
            // Windows: %LOCALAPPDATA%
            // macOS: ~/Library/Application Support
            // Linux: ~/.local/share
            typst_packages_dir: dirs::data_local_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("typst/packages/local")
                .to_string_lossy()
                .to_string(),
        }
    }

}



impl PathConfig {
    /// Resolve all paths to absolute paths
    pub fn resolve_paths(&mut self) -> Result<()> {
        let current_dir = std::env::current_dir()?;

        self.notes_dir = Self::resolve_path(&self.notes_dir, &current_dir)?;
        self.obsidian_dir = Self::resolve_path(&self.obsidian_dir, &current_dir)?;
        self.templates_dir = Self::resolve_path(&self.templates_dir, &current_dir)?;

        Ok(())
    }

    fn resolve_path(path: &str, base: &std::path::Path) -> Result<String> {
        let path_buf = if std::path::Path::new(path).is_absolute() {
            std::path::PathBuf::from(path)
        } else {
            base.join(path)
        };

        if cfg!(windows) {
            // On Windows, build absolute path manually to avoid \\?\ prefix
            let absolute = if path_buf.is_absolute() {
                path_buf
            } else {
                std::env::current_dir()?.join(&path_buf)
            };

            // Convert to string and normalize path separators
            let path_str = absolute.to_string_lossy().to_string();

            // Remove any \\?\ prefix if it somehow got added
            let clean_path = if path_str.starts_with(r"\\?\") {
                path_str[4..].to_string()
            } else {
                path_str
            };

            Ok(clean_path.replace('/', "\\"))
        } else {
            // On Unix, canonicalize is safe
            Ok(path_buf.canonicalize()
                .unwrap_or(path_buf)
                .to_string_lossy()
                .to_string())
        }
    }



}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypstConfig {
    /// Additional compile arguments
    pub compile_args: Vec<String>,

    /// Watch mode arguments
    pub watch_args: Vec<String>,

    /// Whether to clean PDFs before compiling
    pub clean_before_compile: bool,

    /// Output directory for PDFs (relative to source)
    pub output_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    /// Maximum number of search results to show
    pub max_results: usize,

    /// Include file context lines around matches
    pub context_lines: usize,

    /// Case sensitive search
    pub case_sensitive: bool,

    /// File extensions to search in
    pub file_extensions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SemesterFormat {
    /// "2024 Spring", "2024 Fall"
    YearSeason,
    /// "Spring 2024", "Fall 2024"
    SeasonYear,
    /// "S24", "F24"
    ShortForm,
    /// Custom format string
    Custom(String),
}

impl Default for Config {
    fn default() -> Self {
        // Create default courses
        let mut default_courses = std::collections::HashMap::new();

        // Add some common DTU courses as defaults
        let common_courses = [
            ("01005", "Advanced Engineering Mathematics 1"),
            ("01006", "Advanced Engineering Mathematics 2"),
            ("01017", "Discrete Mathematics"),
            ("02101", "Introduction to Programming"),
            ("02102", "Algorithms and Data Structures"),
            ("25200", "Classical Physics 1"),
            ("22100", "Electronics 1"),
        ];

        for (id, name) in common_courses {
            default_courses.insert(id.to_string(), name.to_string());
        }

        Self {
            author: "Your Name".to_string(),
            preferred_editor: None,
            template_version: "0.1.0".to_string(),
            semester_format: SemesterFormat::YearSeason,
            note_preferences: NotePreferences::default(),
            paths: PathConfig::default(),
            typst: TypstConfig::default(),
            search: SearchConfig::default(),
            courses: default_courses,
        }
    }
}

impl Default for NotePreferences {
    fn default() -> Self {
        Self {
            auto_open: true,
            include_date_in_title: true,
            lecture_sections: vec![
                "Key Concepts".to_string(),
                "Mathematical Framework".to_string(),
                "Examples".to_string(),
                "Important Points".to_string(),
                "Questions & Follow-up".to_string(),
                "Connections to Previous Material".to_string(),
                "Next Class Preview".to_string(),
            ],
            assignment_sections: vec![
                "Problem 1".to_string(),
                "Problem 2".to_string(),
                "Problem 3".to_string(),
            ],
            create_backups: false,
        }
    }
}


impl Default for TypstConfig {
    fn default() -> Self {
        Self {
            compile_args: vec![],
            watch_args: vec![],
            clean_before_compile: false,
            output_dir: None,
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_results: 50,
            context_lines: 2,
            case_sensitive: false,
            file_extensions: vec!["typ".to_string(), "md".to_string()],
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            config
        } else {
            // Create default config and save it
            let config = Config::default();
            config.save()?;
            config
        };

        // Resolve relative paths to absolute paths
        config.paths.resolve_paths()?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    /// Get the path to the config file
    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));

        Ok(config_dir.join("dtu-notes").join("config.json"))
    }

    /// Get the config directory path
    pub fn config_dir() -> Result<PathBuf> {
        let config_file = Self::config_file_path()?;
        Ok(config_file.parent().unwrap().to_path_buf())
    }

    /// Update author name
    pub fn set_author(&mut self, author: String) -> Result<()> {
        self.author = author;
        self.save()
    }

    /// Update preferred editor
    pub fn set_editor(&mut self, editor: Option<String>) -> Result<()> {
        self.preferred_editor = editor;
        self.save()
    }

    /// Get formatted semester string
    pub fn format_semester(&self, year: i32, is_spring: bool) -> String {
        match &self.semester_format {
            SemesterFormat::YearSeason => {
                format!("{} {}", year, if is_spring { "Spring" } else { "Fall" })
            }
            SemesterFormat::SeasonYear => {
                format!("{} {}", if is_spring { "Spring" } else { "Fall" }, year)
            }
            SemesterFormat::ShortForm => {
                format!("{}{}", if is_spring { "S" } else { "F" }, year % 100)
            }
            SemesterFormat::Custom(format) => format
                .replace("{year}", &year.to_string())
                .replace("{season}", if is_spring { "Spring" } else { "Fall" })
                .replace("{s}", if is_spring { "S" } else { "F" })
                .replace("{yy}", &format!("{:02}", year % 100)),
        }
    }

    /// Add a course
    pub fn add_course(&mut self, course_id: String, course_name: String) -> Result<()> {
        self.courses.insert(course_id, course_name);
        self.save()
    }

    /// Remove a course
    pub fn remove_course(&mut self, course_id: &str) -> Result<bool> {
        let removed = self.courses.remove(course_id).is_some();
        self.save()?;
        Ok(removed)
    }

    /// Get course name
    pub fn get_course_name(&self, course_id: &str) -> String {
        self.courses.get(course_id).cloned().unwrap_or_default()
    }

    /// List all courses
    pub fn list_courses(&self) -> Vec<(String, String)> {
        let mut courses: Vec<(String, String)> = self
            .courses
            .iter()
            .map(|(id, name)| (id.clone(), name.clone()))
            .collect();
        courses.sort_by(|a, b| a.0.cmp(&b.0));
        courses
    }

    /// Get list of preferred editors in order
    pub fn get_editor_list(&self) -> Vec<String> {
        let mut editors = Vec::new();

        // Add preferred editor first if set
        if let Some(ref preferred) = self.preferred_editor {
            editors.push(preferred.clone());
        }

        // Add default editors based on OS
        if cfg!(windows) {
            editors.extend(["code", "notepad"].iter().map(|s| s.to_string()));
        } else {
            editors.extend(
                ["code", "nvim", "vim", "nano"]
                    .iter()
                    .map(|s| s.to_string()),
            );
        }

        // Remove duplicates while preserving order
        let mut unique_editors = Vec::new();
        for editor in editors {
            if !unique_editors.contains(&editor) {
                unique_editors.push(editor);
            }
        }

        unique_editors
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        if self.author == "Your Name" {
            warnings.push("Author name is set to default value".to_string());
        }

        if self.search.max_results == 0 {
            warnings.push("Max search results is set to 0".to_string());
        }

        // Check if template directory exists
        if !std::path::Path::new(&self.paths.templates_dir).exists() {
            warnings.push(format!(
                "Template directory '{}' doesn't exist",
                self.paths.templates_dir
            ));
        }

        Ok(warnings)
    }
}

/// Helper functions for other modules to use
pub fn get_config() -> Result<Config> {
    Config::load()
}

pub fn update_author(new_author: String) -> Result<()> {
    let mut config = Config::load()?;
    config.set_author(new_author)
}

pub fn update_editor(new_editor: Option<String>) -> Result<()> {
    let mut config = Config::load()?;
    config.set_editor(new_editor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.author, "Your Name");
        assert!(config.note_preferences.auto_open);
    }

    #[test]
    fn test_semester_formatting() {
        let config = Config::default();
        assert_eq!(config.format_semester(2024, true), "2024 Spring");
        assert_eq!(config.format_semester(2024, false), "2024 Fall");
    }

    #[test]
    fn test_editor_list() {
        let mut config = Config::default();
        config.preferred_editor = Some("emacs".to_string());

        let editors = config.get_editor_list();
        assert_eq!(editors[0], "emacs");
    }
}
