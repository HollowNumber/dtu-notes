use anyhow::Result;
use chrono::{Datelike, Local};
use colored::*;
use std::collections::HashMap;
use std::process::Command;
use std::sync::LazyLock;
use crate::config::Config;

static COURSES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    [
        // Mathematics and Computer Science
        ("01005", "Advanced Engineering Mathematics 1"),
        ("01006", "Advanced Engineering Mathematics 2"),
        ("01017", "Discrete Mathematics"),
        ("01035", "Mathematics 1"),
        ("01037", "Mathematics 2"),
        ("02101", "Introduction to Programming"),
        ("02102", "Algorithms and Data Structures"),
        // ... add all your courses here
    ].iter().cloned().collect()
});

pub fn get_current_semester(config: &Config) -> String {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let is_spring = month <= 6;

    config.format_semester(year, is_spring)
}


pub fn get_course_name(course_id: &str) -> String {
    COURSES.get(course_id).unwrap_or(&"").to_string()
}

pub fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .to_lowercase()
}

pub fn validate_course_id(course_id: &str) -> Result<()> {
    if course_id.len() != 5 || !course_id.chars().all(|c| c.is_ascii_digit()) {
        anyhow::bail!("Course ID must be 5 digits (e.g., 02101)");
    }
    Ok(())
}

pub fn open_file(filepath: &str, config: &Config) -> Result<()> {
    // Try to open with system default first
    if let Ok(_) = opener::open(filepath) {
        println!("{} Opened file with system default", "✅".green());
        return Ok(());
    }

    // Fallback to configured editors
    let editors = config.get_editor_list();

    for editor in editors {
        println!("  Trying {}...", editor.dimmed());

        match std::process::Command::new(&editor).arg(filepath).spawn() {
            Ok(mut child) => {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        if status.success() {
                            println!("{} Opened file in {}", "✅".green(), editor);
                            return Ok(());
                        }
                    },
                    Ok(None) => {
                        println!("{} Opened file in {}", "✅".green(), editor);
                        return Ok(());
                    },
                    Err(_) => continue,
                }
            }
            Err(_) => continue,
        }
    }

    println!("{} No suitable editor found. File created at: {}", "⚠️".yellow(), filepath);
    Ok(())
}

