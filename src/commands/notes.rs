//! Note management commands
//!
//! Handles lecture note creation, opening, and listing using core business logic.

use anyhow::Result;
use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::config::get_config;
use crate::core::template_engine::{TemplateEngine, TemplateType};
use crate::core::validation::Validator;
use crate::core::directory_scanner::DirectoryScanner;
use crate::core::file_operations::FileOperations;
use crate::core::status_manager::StatusManager;
use crate::ui::output::{OutputManager, Status};

pub fn create_note(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;

    // Generate template content and filename
    let content = TemplateEngine::generate_lecture_template(course_id, &config, None)?;
    let filename = TemplateEngine::generate_filename(
        course_id,
        &TemplateType::Lecture,
        None
    )?;

    let course_dir = format!("{}/{}/lectures", config.paths.notes_dir, course_id);
    let filepath = format!("{}/{}", course_dir, filename);

    // Create directory structure
    fs::create_dir_all(&course_dir)?;

    if Path::new(&filepath).exists() {
        OutputManager::print_status(
            Status::Warning,
            &format!("Note already exists: {}", filepath)
        );
        println!("Opening existing file...");
    } else {
        OutputManager::print_status(
            Status::Success,
            &format!("Creating new DTU lecture note: {}", filepath)
        );
        fs::write(&filepath, content)?;
    }

    // Open file if configured to do so
    if config.note_preferences.auto_open {
        FileOperations::open_file(&filepath, &config)?;
    } else {
        println!("File created at: {}", filepath);
    }

    Ok(())
}

pub fn open_recent(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;

    let course_dir = format!("{}/{}/lectures", config.paths.notes_dir, course_id);

    if !Path::new(&course_dir).exists() {
        OutputManager::print_status(
            Status::Error,
            &format!("No lectures directory found for course {}", course_id)
        );
        println!("Create your first note with: {}",
                 format!("noter note {}", course_id).bright_white());
        return Ok(());
    }

    // Find most recent file using directory scanner
    let files = DirectoryScanner::scan_directory_for_files(&course_dir, &["typ"])?;

    if let Some(most_recent) = DirectoryScanner::find_most_recent(&files) {
        OutputManager::print_status(
            Status::Info,
            &format!("Opening most recent note: {}",
                     most_recent.path.file_name().unwrap().to_string_lossy().yellow())
        );
        FileOperations::open_file(&most_recent.path.to_string_lossy(), &config)?;
    } else {
        OutputManager::print_status(
            Status::Warning,
            &format!("No lecture notes found for course {}", course_id)
        );
        println!("Create your first note with: {}",
                 format!("noter note {}", course_id).bright_white());
    }

    Ok(())
}

pub fn list_recent(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;
    let course_dir = format!("{}/{}/lectures", config.paths.notes_dir, course_id);

    if !Path::new(&course_dir).exists() {
        OutputManager::print_status(
            Status::Error,
            &format!("Course directory not found: {}", course_dir)
        );
        return Ok(());
    }

    OutputManager::print_section(&format!("Recent notes for {}", course_id), Some("📚"));

    let mut files = DirectoryScanner::scan_directory_for_files(&course_dir, &["typ"])?;

    // Sort by modification time (most recent first)
    files.sort_by(|a, b| b.modified.cmp(&a.modified));

    if files.is_empty() {
        println!("  No notes found");
    } else {
        for file in files.iter().take(10) {
            if let Some(name) = file.path.file_name().and_then(|n| n.to_str()) {
                let datetime: chrono::DateTime<chrono::Local> = file.modified.into();
                println!("  {} - {}",
                         name,
                         datetime.format("%Y-%m-%d %H:%M"));
            }
        }
    }

    Ok(())
}

pub fn create_index(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;

    // Look up course name from config
    let course_name = config.courses.get(course_id)
        .ok_or_else(|| anyhow::anyhow!("Course '{}' not found in config", course_id))?;

    let courses_dir = format!("{}/courses", config.paths.obsidian_dir);
    let index_file = format!("{}/courses/{}-{}.md", config.paths.obsidian_dir, course_id, course_name);
    let semester = StatusManager::get_current_semester(&config);

    if Path::new(&index_file).exists() {
        OutputManager::print_status(
            Status::Warning,
            &format!("Index already exists: {}", index_file)
        );
    } else {
        OutputManager::print_status(
            Status::Success,
            &format!("Creating course index: {}", index_file)
        );

        let content = generate_obsidian_index_content(course_id, course_name, &semester);
        fs::create_dir_all(&courses_dir)?;
        fs::write(&index_file, content)?;
    }

    if config.note_preferences.auto_open {
        let vault_name = Path::new(&config.paths.obsidian_dir)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("vault");
        let obsidian_uri = format!("obsidian://open?vault={}&file=courses/{}-{}.md",
                                   vault_name, course_id, course_name);
        opener::open(obsidian_uri)?;
    } else {
        println!("File created at: {}", index_file);
    }

    Ok(())
}

fn generate_obsidian_index_content(course_id: &str, course_name: &str, semester: &str) -> String {
    format!(r#"# {} - {}

## Course Information
- **Course Code**: {}
- **Semester**: {}
- **University**: Technical University of Denmark (DTU)
- **Professor**: 
- **Credits**: 

## Recent Lectures

## Key Topics

## Assignments

## Connections to Other Courses

## Questions & Review Points

## Resources
- Textbook: 
- Course website: 
- Office hours: 

"#, course_id, course_name, course_id, semester)
}