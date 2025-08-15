//! Assignment creation and management
//!
//! Handles creating assignment files with proper templates and structure.

use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::config::Config;
use crate::core::template_engine::TemplateEngine;
use crate::core::validation::Validator;

pub struct AssignmentManager;

impl AssignmentManager {
    /// Create a new assignment file
    pub fn create_assignment(course_id: &str, title: &str, config: &Config) -> Result<String> {
        Validator::validate_course_id(course_id)?;

        let course_name = config.get_course_name(course_id);
        if course_name.is_empty() {
            anyhow::bail!("Course {} not found in configuration. Add it first with 'noter courses add'", course_id);
        }

        // Create assignment directory if it doesn't exist
        let assignments_dir = Path::new(&config.paths.notes_dir)
            .join(course_id)
            .join("assignments");

        fs::create_dir_all(&assignments_dir)?;

        // Generate filename
        let sanitized_title = Validator::sanitize_filename(title);
        let filename = format!("{}.typ", sanitized_title);
        let file_path = assignments_dir.join(&filename);

        // Check if file already exists
        if file_path.exists() {
            if config.note_preferences.create_backups {
                Self::create_backup(&file_path)?;
            } else {
                anyhow::bail!("Assignment file already exists: {}", file_path.display());
            }
        }

        // Generate content using template engine
        let content = TemplateEngine::generate_assignment_template(course_id, title, config)?;

        // Write file
        fs::write(&file_path, content)?;

        Ok(file_path.to_string_lossy().to_string())
    }

    /// List recent assignments for a course
    pub fn list_recent_assignments(course_id: &str, config: &Config, limit: usize) -> Result<Vec<String>> {
        let assignments_dir = Path::new(&config.paths.notes_dir)
            .join(course_id)
            .join("assignments");

        if !assignments_dir.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        for entry in fs::read_dir(&assignments_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "typ") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        files.push((path.to_string_lossy().to_string(), modified));
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        files.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(files.into_iter().take(limit).map(|(path, _)| path).collect())
    }

    /// Get assignment statistics for a course
    pub fn get_assignment_stats(course_id: &str, config: &Config) -> Result<(usize, Option<std::time::SystemTime>)> {
        let assignments_dir = Path::new(&config.paths.notes_dir)
            .join(course_id)
            .join("assignments");

        if !assignments_dir.exists() {
            return Ok((0, None));
        }

        let mut count = 0;
        let mut most_recent = None;

        for entry in fs::read_dir(&assignments_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "typ") {
                count += 1;

                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        match most_recent {
                            None => most_recent = Some(modified),
                            Some(prev_time) => {
                                if modified > prev_time {
                                    most_recent = Some(modified);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((count, most_recent))
    }

    fn create_backup(file_path: &Path) -> Result<()> {
        let backup_path = file_path.with_extension("typ.bak");
        fs::copy(file_path, backup_path)?;
        Ok(())
    }
}