use anyhow::Result;
use chrono::Local;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::get_config;
use crate::utils::{get_current_semester, open_file, validate_course_id};

pub fn create_note(course_id: &str) -> Result<()> {
    validate_course_id(course_id)?;
    let config = get_config()?;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let course_dir = format!("{}/{}/lectures", config.paths.notes_dir, course_id);
    let filename = format!("{}-{}-lecture.typ", date, course_id);
    let filepath = format!("{}/{}", course_dir, filename);
    let semester = get_current_semester(&config);

    fs::create_dir_all(&course_dir)?;

    if Path::new(&filepath).exists() {
        println!("{} Note already exists: {}", "📝".yellow(), filepath);
        println!("Opening existing file...");
    } else {
        println!("{} Creating new DTU lecture note: {}", "🎓".green(), filepath);

        // Generate sections from config
        let mut sections_content = String::new();
        for section in &config.note_preferences.lecture_sections {
            // Add the section header
            sections_content.push_str(&format!("\n= {}\n", section));

            // Add specific content for certain sections
            if section == "Examples" {
                sections_content.push_str("\n#example[\n  Insert example here...\n]\n");
            } else if section == "Important Points" {
                sections_content.push_str("\n#important[\n  Key takeaways from today's lecture\n]\n");
            } else {
                // For other sections, just add some space for content
                sections_content.push_str("\n\n");
            }
        }


        let title = if config.note_preferences.include_date_in_title {
            format!("Lecture - {}", date)
        } else {
            "Lecture Notes".to_string()
        };

        let content = format!(r#"#import "@local/dtu-template:{}": *

#show: dtu-note.with(
  course: "{}",
  course-name: get-course-name("{}"),
  title: "{}",
  date: datetime.today(),
  author: "{}",
  semester: "{}"
)

= {}
{}
"#, config.template_version, course_id, course_id, title, config.author, semester, title, sections_content);

        fs::write(&filepath, content)?;
    }

    // Only open if auto_open is enabled
    if config.note_preferences.auto_open {
        open_file(&filepath, &config)?;
    } else {
        println!("File created at: {}", filepath);
    }

    Ok(())
}

pub fn open_recent(course_id: &str) -> Result<()> {
    validate_course_id(course_id)?;
    let config = get_config()?;

    let course_dir = format!("{}/{}/lectures", config.paths.notes_dir, course_id);

    if !Path::new(&course_dir).exists() {
        println!("{} No lectures directory found for course {}", "❌".red(), course_id);
        println!("Create your first note with: {}", format!("noter note {}", course_id).bright_white());
        return Ok(());
    }

    // Find the most recent .typ file
    let mut most_recent: Option<(std::path::PathBuf, std::time::SystemTime)> = None;

    for entry in fs::read_dir(&course_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "typ") {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    match &most_recent {
                        None => most_recent = Some((path, modified)),
                        Some((_, prev_time)) => {
                            if modified > *prev_time {
                                most_recent = Some((path, modified));
                            }
                        }
                    }
                }
            }
        }
    }

    match most_recent {
        Some((path, _)) => {
            println!("{} Opening most recent note: {}", "📂".blue(), path.file_name().unwrap().to_str().unwrap().yellow());
            open_file(&path.to_string_lossy(), &config)?;
        }
        None => {
            println!("{} No lecture notes found for course {}", "📝".yellow(), course_id);
            println!("Create your first note with: {}", format!("noter note {}", course_id).bright_white());
        }
    }

    Ok(())
}


pub fn list_recent(course_id: &str) -> Result<()> {
    validate_course_id(course_id)?;
    let config = get_config()?;
    let course_dir = format!("{}/{}/lectures", config.paths.notes_dir, course_id);

    if !Path::new(&course_dir).exists() {
        println!("{} Course directory not found: {}", "❌".red(), course_dir);
        return Ok(());
    }

    println!("{} Recent notes for {}:", "📚".blue(), course_id);

    let mut entries: Vec<_> = fs::read_dir(&course_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .map_or(false, |ext| ext == "typ")
        })
        .collect();

    entries.sort_by_key(|entry| {
        entry.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::UNIX_EPOCH)
    });
    entries.reverse();

    for entry in entries.iter().take(10) {
        if let Some(name) = entry.file_name().to_str() {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let datetime: chrono::DateTime<chrono::Local> = modified.into();
                    println!("  {} - {}", name, datetime.format("%Y-%m-%d %H:%M"));
                }
            }
        }
    }

    if entries.is_empty() {
        println!("  No notes found");
    }

    Ok(())
}

pub fn create_index(course_id: &str) -> Result<()> {
    validate_course_id(course_id)?;
    let config = get_config()?;

    // Look up course name from config
    let course_name = config.courses.get(course_id)
        .ok_or_else(|| anyhow::anyhow!("Course '{}' not found in config", course_id))?;

    let courses_dir = format!("{}/courses", config.paths.obsidian_dir);
    println!("{}", courses_dir.clone());
    let index_file = format!("{}/courses/{}-{}.md", config.paths.obsidian_dir, course_id, course_name);
    let semester = get_current_semester(&config);

    if Path::new(&index_file).exists() {
        println!("{} Index already exists: {}", "📇".yellow(), index_file);
    } else {
        println!("{} Creating course index: {}", "📇".green(), index_file);

        let content = format!(r#"# {} - {}

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

"#, course_id, course_name, course_id, semester);

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
