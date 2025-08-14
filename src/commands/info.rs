use anyhow::Result;
use colored::*;

use crate::config::get_config;
use crate::utils::{get_current_semester};

pub fn list_courses() -> Result<()> {
    let config = get_config()?;
    let courses = config.list_courses();

    if courses.is_empty() {
        println!("{} No courses configured.", "ℹ️".blue());
        println!("Add courses with: {}", "noter add-course 02101 \"Introduction to Programming\"".bright_white());
        return Ok(());
    }

    println!("{} Your DTU Courses:", "🎓".blue());
    println!();

    for (course_id, course_name) in courses {
        println!("  {} - {}", course_id.yellow(), course_name);
    }

    println!();
    println!("{}", "Usage Examples:".green());
    println!("  noter note 02101                           # Create a lecture note");
    println!("  noter assignment 02101 \"Problem Set 1\"     # Create assignment");
    println!("  noter add-course 02101 \"Course Name\"        # Add a new course");
    println!("  noter recent 02101                         # List recent notes for course");

    Ok(())
}

pub fn show_semester() -> Result<()> {
    let config = get_config()?;
    let semester = get_current_semester(&config);

    println!("{} Current semester: {}", "📅".blue(), semester.bright_green());
    println!("{} University: {}", "🏫".blue(), "Technical University of Denmark (DTU)".bright_cyan());

    // Show semester format info
    println!();
    println!("{} Semester format: {:?}", "⚙️".yellow(), config.semester_format);

    // Show some helpful info about the current setup
    println!();
    println!("{} Quick Info:", "ℹ️".blue());
    println!("  Notes directory: {}", config.paths.notes_dir.bright_white());
    println!("  Template version: {}", config.template_version.bright_white());
    println!("  Author: {}", config.author.bright_white());

    Ok(())
}

pub fn show_status() -> Result<()> {
    let config = get_config()?;

    println!("{} DTU Notes Status:", "📊".blue());
    println!();

    // Check if directories exist
    let paths_to_check = [
        ("Notes", &config.paths.notes_dir),
        ("Obsidian Vault", &config.paths.obsidian_dir),
        ("Templates", &config.paths.templates_dir),
        ("Typst Packages", &config.paths.typst_packages_dir),
    ];

    println!("Directory Status:");
    for (name, path) in paths_to_check {
        let exists = std::path::Path::new(path).exists();
        let status = if exists { "✅".green() } else { "❌".red() };
        println!("  {}: {} {}", name, status, path.dimmed());
    }

    // Check template files
    println!();
    println!("Template Status:");
    let template_paths = [
        format!("{}/dtu-template/lib.typ", config.paths.templates_dir),
        format!("{}/dtu-template/{}/lib.typ", config.paths.typst_packages_dir, config.template_version),
        format!("{}/dtu-template/typst.toml", config.paths.templates_dir),
    ];

    for template_path in template_paths {
        let exists = std::path::Path::new(&template_path).exists();
        let status = if exists { "✅".green() } else { "❌".red() };
        println!("  {}: {}", status, template_path.dimmed());
    }

    // Check for course directories
    println!();
    if std::path::Path::new(&config.paths.notes_dir).exists() {
        let course_count = count_course_directories(&config.paths.notes_dir)?;
        println!("Courses initialized: {}", course_count.to_string().bright_green());
    } else {
        println!("Courses initialized: {}", "0 (run setup first)".yellow());
    }

    // Configuration warnings
    let warnings = config.validate()?;
    if !warnings.is_empty() {
        println!();
        println!("{} Configuration Warnings:", "⚠️".yellow());
        for warning in warnings {
            println!("  • {}", warning);
        }
    }

    println!();
    if !std::path::Path::new(&config.paths.notes_dir).exists() {
        println!("{} Run {} to initialize your note-taking environment",
                 "💡".yellow(), "noter setup".bright_white());
    } else {
        println!("{} Ready to take notes! Try {} to get started",
                 "🎉".green(), "noter note 02101".bright_white());
    }

    Ok(())
}

fn count_course_directories(notes_dir: &str) -> Result<usize> {
    let mut count = 0;

    if let Ok(entries) = std::fs::read_dir(notes_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    // Check if it looks like a course code (5 digits)
                    if let Some(name) = entry.file_name().to_str() {
                        if name.len() == 5 && name.chars().all(|c| c.is_ascii_digit()) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(count)
}



pub fn show_enhanced_status() -> Result<()> {
    let config = get_config()?;

    println!("{} DTU Notes Status Dashboard:", "📊".blue());
    println!();

    // Basic system status (reuse existing logic)
    show_system_status(&config)?;

    // New enhanced sections
    show_activity_summary(&config)?;
    show_course_health(&config)?;
    show_quick_suggestions(&config)?;

    Ok(())
}

fn show_system_status(config: &crate::config::Config) -> Result<()> {
    // Check if directories exist
    let paths_to_check = [
        ("Notes", &config.paths.notes_dir),
        ("Obsidian Vault", &config.paths.obsidian_dir),
        ("Templates", &config.paths.templates_dir),
        ("Typst Packages", &config.paths.typst_packages_dir),
    ];

    println!("🏗️ System Status:");
    for (name, path) in paths_to_check {
        let exists = std::path::Path::new(path).exists();
        let status = if exists { "✅".green() } else { "❌".red() };
        println!("  {}: {} {}", name, status, path.dimmed());
    }

    println!();
    Ok(())
}

fn show_activity_summary(config: &crate::config::Config) -> Result<()> {
    println!("📈 Recent Activity:");

    if !std::path::Path::new(&config.paths.notes_dir).exists() {
        println!("  No activity (run setup first)");
        println!();
        return Ok(());
    }

    let mut total_notes = 0;
    let mut total_assignments = 0;
    let mut most_recent_file: Option<(String, std::time::SystemTime, String)> = None;
    let mut course_activity: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    // Scan all course directories
    for entry in std::fs::read_dir(&config.paths.notes_dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(course_id) = entry.file_name().to_str() {
                if course_id.len() == 5 && course_id.chars().all(|c| c.is_ascii_digit()) {
                    let (notes, assignments, recent) = scan_course_directory(&entry.path())?;
                    total_notes += notes;
                    total_assignments += assignments;
                    course_activity.insert(course_id.to_string(), notes + assignments);

                    if let Some((file, time)) = recent {
                        match &most_recent_file {
                            None => most_recent_file = Some((file, time, course_id.to_string())),
                            Some((_, prev_time, _)) => {
                                if time > *prev_time {
                                    most_recent_file = Some((file, time, course_id.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some((file, time, course_id)) = most_recent_file {
        let datetime: chrono::DateTime<chrono::Local> = time.into();
        let course_name = config.courses.get(&course_id).cloned().unwrap_or_default();
        println!("  Last activity: {} ({} - {})",
                 datetime.format("%Y-%m-%d %H:%M").to_string().bright_white(),
                 course_id.yellow(),
                 if course_name.is_empty() { "Unknown Course".to_string() } else { course_name });
        println!("  File: {}", file.dimmed());
    }

    println!("  Total files: {} notes, {} assignments",
             total_notes.to_string().green(),
             total_assignments.to_string().blue());

    // Most active course
    if let Some((course_id, count)) = course_activity.iter().max_by_key(|&(_, count)| count) {
        let course_name = config.courses.get(course_id).cloned().unwrap_or_default();
        println!("  Most active: {} ({} files) - {}",
                 course_id.yellow(),
                 count.to_string().green(),
                 course_name.dimmed());
    }

    println!();
    Ok(())
}

fn show_course_health(config: &crate::config::Config) -> Result<()> {
    println!("🎓 Course Health:");

    if !std::path::Path::new(&config.paths.notes_dir).exists() {
        println!("  No courses found");
        println!();
        return Ok(());
    }

    let mut courses_with_activity = Vec::new();

    for (course_id, course_name) in &config.courses {
        let course_path = std::path::PathBuf::from(&config.paths.notes_dir).join(course_id);
        if course_path.exists() {
            let (notes, assignments, recent) = scan_course_directory(&course_path)?;
            let days_since_last = if let Some((_, time)) = recent {
                let duration = std::time::SystemTime::now().duration_since(time).unwrap_or_default();
                duration.as_secs() / (24 * 60 * 60)
            } else {
                999 // Never used
            };

            courses_with_activity.push((course_id.clone(), course_name.clone(), notes, assignments, days_since_last));
        }
    }

    courses_with_activity.sort_by_key(|&(_, _, _, _, days)| days);

    for (course_id, course_name, notes, assignments, days_since_last) in courses_with_activity {
        let health_indicator = match (notes + assignments, days_since_last) {
            (0, _) => "❌",
            (_, 0..=3) => "✅",
            (_, 4..=7) => "⚠️",
            _ => "🔴",
        };

        let last_activity = match days_since_last {
            0 => "today".bright_green(),
            1 => "1 day ago".green(),
            2..=7 => format!("{} days ago", days_since_last).yellow(),
            8..=14 => format!("{} days ago", days_since_last).red(),
            999 => "never".red(),
            _ => format!("{} days ago", days_since_last).red(),
        };

        println!("  {} {} - {} ({} notes, {} assignments, last: {})",
                 health_indicator,
                 course_id.yellow(),
                 course_name.dimmed(),
                 notes,
                 assignments,
                 last_activity);
    }

    println!();
    Ok(())
}

fn show_quick_suggestions(config: &crate::config::Config) -> Result<()> {
    println!("💡 Quick Suggestions:");

    // Find most active course for suggestions
    let mut most_active_course: Option<String> = None;
    let mut max_activity = 0;

    if std::path::Path::new(&config.paths.notes_dir).exists() {
        for (course_id, _) in &config.courses {
            let course_path = std::path::PathBuf::from(&config.paths.notes_dir).join(course_id);
            if course_path.exists() {
                let (notes, assignments, _) = scan_course_directory(&course_path)?;
                let total = notes + assignments;
                if total > max_activity {
                    max_activity = total;
                    most_active_course = Some(course_id.clone());
                }
            }
        }
    }

    if let Some(course_id) = most_active_course {
        println!("  • {} (continue with most active course)", format!("noter note {}", course_id).bright_white());
        println!("  • {} (open recent note)", format!("noter open {}", course_id).bright_white());
    } else if let Some((course_id, _)) = config.courses.iter().next() {
        println!("  • {} (create your first note)", format!("noter note {}", course_id).bright_white());
    } else {
        println!("  • {} (add your first course)", "noter courses add 02101 \"Course Name\"".bright_white());
    }

    println!("  • {} (see all recent activity)", "noter recent".bright_white());
    println!("  • {} (manage courses)", "noter courses list".bright_white());

    println!();
    Ok(())
}

// Helper function to scan a course directory
fn scan_course_directory(course_path: &std::path::Path) -> Result<(usize, usize, Option<(String, std::time::SystemTime)>)> {
    let mut notes = 0;
    let mut assignments = 0;
    let mut most_recent: Option<(String, std::time::SystemTime)> = None;

    // Check lectures directory
    let lectures_path = course_path.join("lectures");
    if lectures_path.exists() {
        for entry in std::fs::read_dir(&lectures_path)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "typ") {
                notes += 1;
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        match &most_recent {
                            None => most_recent = Some((filename, modified)),
                            Some((_, prev_time)) => {
                                if modified > *prev_time {
                                    most_recent = Some((filename, modified));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check assignments directory
    let assignments_path = course_path.join("assignments");
    if assignments_path.exists() {
        for entry in std::fs::read_dir(&assignments_path)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "typ") {
                assignments += 1;
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        match &most_recent {
                            None => most_recent = Some((filename, modified)),
                            Some((_, prev_time)) => {
                                if modified > *prev_time {
                                    most_recent = Some((filename, modified));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok((notes, assignments, most_recent))
}
