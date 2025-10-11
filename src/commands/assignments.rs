//! Assignment command implementation
//!
//! Uses the template system directly for assignment creation and management.

use anyhow::Result;
use colored::Colorize;

use std::path::{Path, PathBuf};

use crate::config::get_config;
use crate::core::directory_scanner::DirectoryScanner;
use crate::core::file_operations::FileOperations;
use crate::core::template::{builder::TemplateBuilder, engine::TemplateReference};
use crate::core::validation::Validator;
use crate::ui::output::{OutputManager, Status};

/// Create a new assignment using the template system
pub fn create_assignment(course_id: &str, title: &str) -> Result<()> {
    let config = get_config()?;

    // Validate course ID
    Validator::validate_course_id(course_id)?;

    let course_name = config.get_course_name(course_id);
    if course_name.is_empty() {
        OutputManager::print_status(
            Status::Error,
            &format!(
                "Course {} not found in configuration. Add it first with 'noter courses add'",
                course_id
            ),
        );
        return Ok(());
    }

    OutputManager::print_status(
        Status::Loading,
        &format!("Creating assignment for course {}", course_id.yellow()),
    );

    // Create assignment directory if it doesn't exist
    let assignments_dir = config.get_assignments_dir(course_id);

    FileOperations::ensure_directory_exists(&assignments_dir)?;

    // Generate filename
    let sanitized_title = Validator::sanitize_filename(title);
    let filename = format!("{}.typ", sanitized_title);
    let file_path = assignments_dir.join(&filename);

    // Generate content using the template system
    match TemplateBuilder::new(course_id, &config)?
        .with_reference(TemplateReference::assignment())
        .with_title(title)
        .with_sections(config.note_preferences.assignment_sections.clone())
        .build()
    {
        Ok(content) => {
            // Write file
            FileOperations::create_file_with_content_and_open(&file_path, &content, &config)?;

            // Show helpful next steps
            println!();
            OutputManager::print_command_examples(&[
                (
                    &format!("noter compile {}", file_path.to_string_lossy()),
                    "Compile to PDF",
                ),
                (
                    &format!("noter watch {}", file_path.to_string_lossy()),
                    "Auto-compile on changes",
                ),
                (&format!("noter recent {}", course_id), "List recent files"),
            ]);
        }
        Err(e) => {
            OutputManager::print_status(
                Status::Error,
                &format!("Failed to generate assignment template: {}", e),
            );

            if e.to_string().contains("template") || e.to_string().contains("Template") {
                OutputManager::print_error_with_context(
                    "Template error occurred",
                    &[
                        "Templates haven't been installed yet",
                        "Template configuration is missing or invalid",
                    ],
                );
                println!("Try: {}", "noter template update".bright_white());
            }
        }
    }

    Ok(())
}

/// List recent assignments for a course
pub fn list_recent_assignments(course_id: &str, limit: usize) -> Result<()> {
    let config = get_config()?;

    // Validate course ID
    Validator::validate_course_id(course_id)?;

    OutputManager::print_status(
        Status::Loading,
        &format!("Finding recent assignments for {}", course_id.yellow()),
    );

    let assignments_dir = config.get_assignments_dir(course_id);

    if !assignments_dir.exists() {
        OutputManager::print_empty_state(
            &format!("No assignments found for course {}", course_id),
            &format!("noter assignment {} \"Assignment Title\"", course_id),
            "Create one",
        );
        return Ok(());
    }

    // Use DirectoryScanner to list .typ files
    let typ_files = DirectoryScanner::list_files_with_extensions(&assignments_dir, &["typ"])?;

    if typ_files.is_empty() {
        OutputManager::print_empty_state(
            &format!("No assignments found for course {}", course_id),
            &format!("noter assignment {} \"Assignment Title\"", course_id),
            "Create one",
        );
        return Ok(());
    }

    // Collect files with modification times using FileOperations
    let mut files: Vec<(PathBuf, std::time::SystemTime)> = typ_files
        .into_iter()
        .filter_map(|path| {
            FileOperations::get_modification_time(&path)
                .ok()
                .map(|modified| (path, modified))
        })
        .collect();

    // Sort by modification time (newest first)
    files.sort_by(|a, b| b.1.cmp(&a.1));

    // Take the most recent files up to the limit
    let recent_assignments: Vec<PathBuf> = files
        .into_iter()
        .take(limit)
        .map(|(path, _)| path)
        .collect();

    OutputManager::print_section(
        &format!("Recent assignments for {}", course_id.yellow()),
        Some("📝"),
    );

    for (i, assignment_path) in recent_assignments.iter().enumerate() {
        let file_name = assignment_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        OutputManager::print_numbered_item(
            i + 1,
            &file_name,
            Some(&assignment_path.display().to_string()),
        );
    }

    println!();
    OutputManager::print_command_examples(&[
        (&format!("noter open {}", course_id), "Open most recent"),
        (
            &format!("noter assignment {} \"New Assignment\"", course_id),
            "Create new assignment",
        ),
    ]);

    Ok(())
}

/// Show assignment statistics for a course
pub fn show_assignment_stats(course_id: &str) -> Result<()> {
    let config = get_config()?;

    // Validate course ID
    Validator::validate_course_id(course_id)?;

    OutputManager::print_status(
        Status::Loading,
        &format!("Calculating assignment stats for {}", course_id.yellow()),
    );

    let assignments_dir = config.get_assignments_dir(course_id);

    let (count, last_modified) = if !assignments_dir.exists() {
        (0, None)
    } else {
        get_assignment_stats_for_directory(&assignments_dir)?
    };

    OutputManager::print_section(
        &format!("Assignment Statistics for {}", course_id.yellow()),
        Some("📊"),
    );

    OutputManager::print_summary("Total assignments", &count.to_string(), "green");

    if let Some(last_modified) = last_modified {
        let datetime: chrono::DateTime<chrono::Local> = last_modified.into();
        println!(
            "Last modified: {}",
            datetime.format("%Y-%m-%d %H:%M").to_string().bright_white()
        );

        let now = std::time::SystemTime::now();
        if let Ok(duration) = now.duration_since(last_modified) {
            let days = duration.as_secs() / (24 * 60 * 60);
            let health = match days {
                0..=3 => format!("{} Excellent - recent activity", "🟢".green()),
                4..=7 => format!("{} Good - somewhat recent", "🟡".yellow()),
                8..=14 => format!("{} Warning - getting old", "🟠".yellow()),
                _ => format!("{} Critical - very old", "🔴".red()),
            };
            println!("Activity health: {}", health);
        }
    } else {
        println!("Last modified: {}", "Never".dimmed());
        println!(
            "Activity health: {}",
            format!("{} Critical - no assignments", "🔴".red())
        );
    }

    println!();
    OutputManager::print_command_examples(&[
        (
            &format!("noter assignments recent {}", course_id),
            "List recent assignments",
        ),
        (
            &format!("noter assignment {} \"New Assignment\"", course_id),
            "Create new assignment",
        ),
    ]);

    Ok(())
}

/// List all assignments across courses with activity summary
pub fn list_all_assignments() -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(Status::Loading, "Scanning all assignments...");

    let mut total_assignments = 0;
    let mut course_assignments = Vec::new();

    for (course_id, course_name) in config.list_courses() {
        let assignments_dir = config.get_assignments_dir(&course_id);

        if let Ok((count, last_modified)) = get_assignment_stats_for_directory(&assignments_dir) {
            total_assignments += count;
            if count > 0 {
                course_assignments.push((course_id, course_name, count, last_modified));
            }
        }
    }

    OutputManager::print_section("Assignment Summary", Some("📋"));

    if total_assignments == 0 {
        OutputManager::print_empty_state(
            "No assignments found.",
            "noter assignment 02101 \"Problem Set 1\"",
            "Create your first assignment with",
        );
        return Ok(());
    }

    OutputManager::print_summary("Total assignments", &total_assignments.to_string(), "green");
    println!();

    // Sort by most recent activity
    course_assignments.sort_by(|a, b| match (a.3, b.3) {
        (Some(a_time), Some(b_time)) => b_time.cmp(&a_time),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.0.cmp(&b.0),
    });

    OutputManager::print_info_line("📚", "Assignments by Course:");
    for (course_id, course_name, count, last_modified) in course_assignments {
        let activity_indicator = if let Some(last_modified) = last_modified {
            let now = std::time::SystemTime::now();
            if let Ok(duration) = now.duration_since(last_modified) {
                let days = duration.as_secs() / (24 * 60 * 60);
                match days {
                    0..=3 => "🟢".to_string(),
                    4..=7 => "🟡".to_string(),
                    8..=14 => "🟠".to_string(),
                    _ => "🔴".to_string(),
                }
            } else {
                "❓".to_string()
            }
        } else {
            "⚫".to_string()
        };

        println!(
            "  {} {} - {} ({})",
            activity_indicator,
            course_id.bright_blue(),
            course_name,
            format!("{} assignments", count).dimmed()
        );
    }

    println!();
    OutputManager::print_command_examples(&[
        (
            "noter assignments recent 02101",
            "Recent assignments for course",
        ),
        ("noter assignments stats 02101", "Detailed stats for course"),
        ("noter assignments health", "Assignment health analysis"),
    ]);

    Ok(())
}

/// Show assignment health and activity analysis
pub fn show_assignment_health(course_id: Option<&str>) -> Result<()> {
    let config = get_config()?;

    let message = if let Some(course_id) = course_id {
        format!("Analyzing assignment health for {}", course_id.yellow())
    } else {
        "Analyzing assignment health for all courses".to_string()
    };

    OutputManager::print_status(Status::Loading, &message);

    let mut health_data = Vec::new();

    let courses_to_check = if let Some(specific_course) = course_id {
        vec![(
            specific_course.to_string(),
            config
                .courses
                .get(specific_course)
                .cloned()
                .unwrap_or_else(|| "Unknown Course".to_string()),
        )]
    } else {
        config.list_courses()
    };

    for (course_id, course_name) in courses_to_check {
        let assignments_dir = config.get_assignments_dir(&course_id);

        if let Ok((count, last_modified)) = get_assignment_stats_for_directory(&assignments_dir) {
            let health_status = calculate_assignment_health_status(count, last_modified);
            let days_since_activity = if let Some(last_modified) = last_modified {
                let now = std::time::SystemTime::now();
                now.duration_since(last_modified)
                    .map(|d| d.as_secs() / (24 * 60 * 60))
                    .unwrap_or(999)
            } else {
                999 // No activity
            };

            health_data.push((
                course_id,
                course_name,
                count,
                days_since_activity,
                health_status,
            ));
        }
    }

    if health_data.is_empty() {
        OutputManager::print_status(Status::Info, "No assignment data found.");
        return Ok(());
    }

    OutputManager::print_section("Assignment Health Analysis", Some("🏥"));

    // Sort by health status and activity
    health_data.sort_by(|a, b| {
        // Sort by health (0 = best, 3 = worst), then by days since activity
        let health_cmp = health_status_to_priority(a.4).cmp(&health_status_to_priority(b.4));
        if health_cmp == std::cmp::Ordering::Equal {
            a.3.cmp(&b.3) // Less days is better
        } else {
            health_cmp
        }
    });

    // Clone health_data for recommendations before consuming it
    let critical_courses: Vec<_> = health_data
        .iter()
        .filter(|(_, _, _, _, health)| *health >= 3)
        .map(|(course_id, _, count, _, health)| (course_id.clone(), *count, *health))
        .collect();

    for (course_id, course_name, count, days_since, health_status) in health_data {
        let (icon, status_text, color_fn): (_, _, fn(&str) -> colored::ColoredString) =
            match health_status {
                0 => ("🟢", "Excellent", |s: &str| s.bright_green()),
                1 => ("🟡", "Good", |s: &str| s.bright_yellow()),
                2 => ("🟠", "Warning", |s: &str| s.yellow()),
                _ => ("🔴", "Critical", |s: &str| s.bright_red()),
            };

        let activity_text = if days_since >= 999 {
            "no activity".dimmed()
        } else if days_since == 0 {
            "active today".bright_green()
        } else if days_since == 1 {
            "active yesterday".green()
        } else {
            format!("active {} days ago", days_since).dimmed()
        };

        println!(
            "  {} {} {} - {} ({} assignments, {})",
            icon,
            color_fn(status_text),
            course_id.bright_blue(),
            course_name,
            count,
            activity_text
        );
    }

    println!();

    // Provide recommendations
    if !critical_courses.is_empty() {
        let recommendations: Vec<(String, String)> = critical_courses
            .iter()
            .map(|(course_id, count, _)| {
                if *count == 0 {
                    (
                        format!("Create first assignment for {}", course_id.bright_blue()),
                        format!("noter assignment {} \"Assignment 1\"", course_id),
                    )
                } else {
                    (
                        format!("Resume work on {}", course_id.bright_blue()),
                        format!("noter assignments recent {}", course_id),
                    )
                }
            })
            .collect();

        OutputManager::print_recommendations(&recommendations);
    }

    OutputManager::print_command_examples(&[
        ("noter assignments list", "Overview of all assignments"),
        (
            "noter assignments recent 02101",
            "Recent assignments for course",
        ),
        (
            "noter assignment 02101 \"New Assignment\"",
            "Create new assignment",
        ),
    ]);

    Ok(())
}

// Helper functions

/// Get assignment statistics for a directory
fn get_assignment_stats_for_directory(
    assignments_dir: &Path,
) -> Result<(usize, Option<std::time::SystemTime>)> {
    if !assignments_dir.exists() {
        return Ok((0, None));
    }

    // Use DirectoryScanner to get file info with metadata
    let files = DirectoryScanner::scan_directory_for_files(assignments_dir, &["typ"])?;
    let count = files.len();

    // Find most recent file
    let most_recent =
        DirectoryScanner::find_most_recent(&files).map(|file_info| file_info.modified);

    Ok((count, most_recent))
}

fn calculate_assignment_health_status(
    count: usize,
    last_modified: Option<std::time::SystemTime>,
) -> usize {
    if count == 0 {
        return 3; // Critical - no assignments
    }

    if let Some(last_modified) = last_modified {
        let now = std::time::SystemTime::now();
        if let Ok(duration) = now.duration_since(last_modified) {
            let days = duration.as_secs() / (24 * 60 * 60);
            match days {
                0..=3 => 0,  // Excellent
                4..=7 => 1,  // Good
                8..=14 => 2, // Warning
                _ => 3,      // Critical
            }
        } else {
            3 // Critical - time error
        }
    } else {
        3 // Critical - no timestamp
    }
}

fn health_status_to_priority(health: usize) -> usize {
    health // 0 = best, 3 = worst
}
