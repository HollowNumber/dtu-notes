use anyhow::Result;
use chrono::{Datelike, Local};
use colored::*;
use std::collections::HashMap;
use std::process::Command;
use std::sync::LazyLock;
use crate::config::Config;


pub fn get_current_semester(config: &Config) -> String {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let is_spring = month <= 6;

    config.format_semester(year, is_spring)
}


pub fn get_course_name(course_id: &str) -> String {
    // Try to get config and check user's courses first
    if let Ok(config) = crate::config::get_config() {
        if let Some(name) = config.courses.get(course_id) {
            return name.clone();
        }
    }

    // Fallback to common DTU courses
    crate::data::get_course_name(course_id)
}

pub fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            // Allow standard ASCII alphanumeric and basic punctuation
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,

            // Convert Danish/Nordic characters to ASCII equivalents
            'æ' | 'Æ' => 'a',
            'ø' | 'Ø' => 'o',
            'å' | 'Å' => 'a',
            'ä' | 'Ä' => 'a',
            'ö' | 'Ö' => 'o',

            // Convert common special characters
            ' ' => '-',  // Spaces to dashes
            '.' => '-',  // Dots to dashes (but preserve at end for extensions)
            ',' => '-',
            ';' => '-',
            ':' => '-',
            '/' | '\\' => '-',  // Slashes to dashes

            // Replace any other problematic character with dash
            _ => '-',
        })
        .collect::<String>()
        // Clean up multiple consecutive dashes
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .to_lowercase()
        // Ensure it doesn't end with a dash
        .trim_end_matches('-')
        .to_string()
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

