//! Note management commands
//!
//! Handles lecture note creation, opening, and listing using core business logic.

use crate::config::get_config;
use crate::core::directory_scanner::DirectoryScanner;
use crate::core::file_operations::FileOperations;
use crate::core::status_manager::StatusManager;
use crate::core::template::{builder::TemplateBuilder, engine::TemplateReference};
use crate::core::validation::Validator;
use crate::ui::output::{OutputManager, Status};
use anyhow::Result;
use colored::Colorize;

pub fn create_note(
    course_id: &str,
    title: &Option<String>,
    variant: &Option<String>,
    sections: &Option<String>,
) -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(Status::Loading, "Creating lecture note...");

    // Generate the title as an owned String to avoid borrowing issues
    let note_title = match title {
        Some(title) => title.clone(),
        None => format!("Lecture - {}", chrono::Local::now().format("%B %d, %Y")),
    };

    // Generate content using builder
    let mut builder = TemplateBuilder::new(course_id, &config)?
        .with_title(&note_title)
        .with_reference(match variant {
            Some(variant) => TemplateReference::lecture().with_variant(variant),
            None => TemplateReference::lecture(),
        });

    builder = match sections {
        None => builder,
        Some(sects) => {
            let sections_to_use = sects
                .split(",")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            builder.with_sections(sections_to_use)
        }
    };

    // Build the template content
    let content = builder.build()?;

    // Generate filename and save
    let variant = variant.clone().unwrap_or_else(|| String::from("lecture"));
    let filename = FileOperations::generate_filename(&course_id, &variant, title.as_deref());

    // File operations
    let filepath = config.get_lectures_dir(course_id).join(&filename);

    FileOperations::create_file_with_content_and_open(&filepath, &content, &config)?;

    Ok(())
}

pub fn open_recent(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;

    let course_dir = config.get_lectures_dir(course_id);

    if !course_dir.exists() {
        OutputManager::print_status(
            Status::Error,
            &format!("No lectures directory found for course {}", course_id),
        );
        println!(
            "Create your first note with: {}",
            format!("noter note {}", course_id).bright_white()
        );
        return Ok(());
    }

    // Find most recent file using directory scanner
    let files = DirectoryScanner::scan_directory_for_files(&course_dir, &["typ"])?;

    if let Some(most_recent) = DirectoryScanner::find_most_recent(&files) {
        OutputManager::print_status(
            Status::Info,
            &format!(
                "Opening most recent note: {}",
                most_recent
                    .path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .yellow()
            ),
        );
        FileOperations::open_path(&most_recent.path, &config)?;
    } else {
        OutputManager::print_status(
            Status::Warning,
            &format!("No lecture notes found for course {}", course_id),
        );
        println!(
            "Create your first note with: {}",
            format!("noter note {}", course_id).bright_white()
        );
    }

    Ok(())
}

pub fn list_recent(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;
    let course_dir = config.get_lectures_dir(course_id);

    if !course_dir.exists() {
        OutputManager::print_status(
            Status::Error,
            &format!("Course directory not found: {}", course_dir.display()),
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
                println!("  {} - {}", name, datetime.format("%Y-%m-%d %H:%M"));
            }
        }
    }

    Ok(())
}

pub fn create_index(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;
    let config = get_config()?;

    // Look up course name from config
    let course_name = config
        .courses
        .get(course_id)
        .ok_or_else(|| anyhow::anyhow!("Course '{}' not found in config", course_id))?;

    let courses_dir = config.get_obsidian_dir_path().join("courses");

    // let index_file = format!(
    //     "{}/courses/{}-{}.md",
    //     config.paths.obsidian_dir, course_id, course_name
    // );

    let index_file = courses_dir.join(format!("{}-{}.md", course_id, course_name));

    let semester = StatusManager::get_current_semester(&config);

    if index_file.exists() {
        OutputManager::print_status(
            Status::Warning,
            &format!("Index already exists: {}", index_file.display()),
        );
    } else {
        OutputManager::print_status(
            Status::Success,
            &format!("Creating course index: {}", index_file.display()),
        );

        let content = generate_obsidian_index_content(course_id, course_name, &semester);
        FileOperations::create_file_with_content(&index_file, &content, &config)?;
    }

    if config.note_preferences.auto_open_file {
        FileOperations::open_obsidian_file(
            &config.get_obsidian_dir_path(),
            &format!("courses/{}-{}.md", course_id, course_name),
        )?;
    } else {
        println!("File created at: {}", index_file.display());
    }

    Ok(())
}

fn generate_obsidian_index_content(course_id: &str, course_name: &str, semester: &str) -> String {
    format!(
        r#"# {} - {}

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

"#,
        course_id, course_name, course_id, semester
    )
}
