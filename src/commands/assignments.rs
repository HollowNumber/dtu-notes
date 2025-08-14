use anyhow::Result;
use chrono::Local;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::get_config;
use crate::utils::{get_current_semester, sanitize_filename, open_file, validate_course_id};

pub fn create_assignment(course_id: &str, title: &str) -> Result<()> {
    validate_course_id(course_id)?;
    let config = get_config()?;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let course_dir = format!("{}/{}/assignments", config.paths.notes_dir, course_id);
    let clean_title = sanitize_filename(title);
    let filename = format!("{}-{}-{}.typ", date, course_id, clean_title);
    let filepath = format!("{}/{}", course_dir, filename);
    let semester = get_current_semester(&config);

    fs::create_dir_all(&course_dir)?;

    if Path::new(&filepath).exists() {
        println!("{} Assignment already exists: {}", "📄".yellow(), filepath);
    } else {
        println!("{} Creating new DTU assignment: {}", "🎓".green(), filepath);

        // Generate sections from config
        let mut sections_content = String::new();
        for (i, section) in config.note_preferences.assignment_sections.iter().enumerate() {
            if i == 0 {
                sections_content.push_str(&"\n==\n\n#note-box[\n  Remember to show all work and explain your reasoning.\n]\n".to_string());
            } else {
                sections_content.push_str(&"\n==\n\n\n".to_string());
            }
        }

        let content = format!(r#"#import "@local/dtu-template:{}": *

#show: dtu-assignment.with(
  course: "{}",
  course-name: get-course-name("{}"),
  title: "{}",
  due-date: datetime.today(),
  author: "{}",
  semester: "{}"
)

= {}
{}
"#, config.template_version, course_id, course_id, title, config.author, semester, title, sections_content);

        fs::write(&filepath, content)?;
    }

    if config.note_preferences.auto_open {
        open_file(&filepath, &config)?;
    } else {
        println!("File created at: {}", filepath);
    }

    Ok(())
}