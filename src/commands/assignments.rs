//! Assignment command implementation
//!
//! Thin command layer that delegates to core assignment manager.

use anyhow::Result;
use colored::Colorize;

use crate::config::get_config;
use crate::core::assignment_manager::AssignmentManager;
use crate::ui::output::{OutputManager, Status};
use crate::core::file_operations;
use crate::core::file_operations::FileOperations;

pub fn create_assignment(course_id: &str, title: &str) -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(
        Status::Loading,
        &format!("Creating assignment for course {}", course_id.yellow())
    );

    match AssignmentManager::create_assignment(course_id, title, &config) {
        Ok(file_path) => {
            OutputManager::print_status(
                Status::Success,
                &format!("Assignment created: {}", file_path.bright_white())
            );

            // Auto-open if configured
            if config.note_preferences.auto_open {
                OutputManager::print_status(Status::Info, "Opening in editor...");
                if let Err(e) = FileOperations::open_file(&file_path, &config) {
                    OutputManager::print_status(
                        Status::Warning,
                        &format!("Could not open file automatically: {}", e)
                    );
                }
            }

            // Show helpful next steps
            println!();
            OutputManager::print_command_examples(&[
                (&format!("noter compile {}", file_path), "Compile to PDF"),
                (&format!("noter watch {}", file_path), "Auto-compile on changes"),
                (&format!("noter recent {}", course_id), "List recent files"),
            ]);
        }
        Err(e) => {
            OutputManager::print_status(Status::Error, &e.to_string());

            if e.to_string().contains("not found in configuration") {
                println!("Add the course first: {}",
                         format!("noter courses add {} \"Course Name\"", course_id).bright_white());
            }
        }
    }

    Ok(())
}