//! Command execution and routing
//!
//! This module serves as the main entry point for all CLI commands,
//! routing them to appropriate command handlers while maintaining
//! clean separation between command parsing and business logic.

use anyhow::{Context, Result};

pub mod notes;
pub mod assignments;
pub mod typst;
pub mod search;
pub mod setup;
pub mod info;
pub mod config_cmd;
pub mod courses;

use crate::{Commands, ConfigAction, CourseAction};

/// Execute a command with proper error context
pub fn execute_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Note { course_id } => {
            notes::create_note(course_id)
                .with_context(|| format!("Failed to create note for course {}", course_id))
        }
        Commands::Assignment { course_id, title } => {
            assignments::create_assignment(course_id, title)
                .with_context(|| format!("Failed to create assignment '{}' for course {}", title, course_id))
        }
        Commands::Compile { filepath } => {
            typst::compile_file(filepath)
                .with_context(|| format!("Failed to compile file: {}", filepath))
        }
        Commands::Watch { filepath } => {
            typst::watch_file(filepath)
                .with_context(|| format!("Failed to watch file: {}", filepath))
        }
        Commands::Recent { course_id } => {
            notes::list_recent(course_id)
                .with_context(|| format!("Failed to list recent notes for course {}", course_id))
        }
        Commands::Setup => {
            setup::setup_repository()
                .with_context(|| "Failed to setup repository")
        }
        Commands::Index { course_id } => {
            notes::create_index(course_id)
                .with_context(|| format!("Failed to create index for course {}", course_id))
        }
        Commands::Search { query } => {
            search::search_notes(query)
                .with_context(|| format!("Failed to search for: {}", query))
        }
        Commands::Courses { action } => {
            execute_course_action(action)
                .with_context(|| "Failed to execute course command")
        }
        Commands::Clean => {
            typst::clean_files()
                .with_context(|| "Failed to clean compiled files")
        }
        Commands::Status => {
            info::show_enhanced_status()
                .with_context(|| "Failed to show status information")
        }
        Commands::Open { course_id } => {
            notes::open_recent(course_id)
                .with_context(|| format!("Failed to open recent note for course {}", course_id))
        }
        Commands::Semester => {
            info::show_semester()
                .with_context(|| "Failed to show semester information")
        }
        Commands::Config { action } => {
            execute_config_action(action)
                .with_context(|| "Failed to execute config command")
        }
    }
}

fn execute_config_action(action: &ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Show => config_cmd::show_config(),
        ConfigAction::SetAuthor { name } => config_cmd::set_author(name),
        ConfigAction::SetEditor { editor } => config_cmd::set_editor(editor),
        ConfigAction::Reset => config_cmd::reset_config(),
        ConfigAction::Path => config_cmd::show_config_path(),
        ConfigAction::Check => config_cmd::check_config(),
    }
}

fn execute_course_action(action: &CourseAction) -> Result<()> {
    match action {
        CourseAction::List => courses::list_courses(),
        CourseAction::Add { course_id, course_name } => courses::add_course(course_id, course_name),
        CourseAction::Remove { course_id } => courses::remove_course(course_id),
        CourseAction::Browse => courses::browse_common_courses(),
    }
}