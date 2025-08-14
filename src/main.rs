mod commands;
mod utils;
mod config;
mod data;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "noter")]
#[command(about = "DTU note-taking CLI with official branding")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new lecture note
    #[command(alias = "n")]
    Note {
        /// Course code (e.g., 02101)
        course_id: String,
    },
    /// Create a new assignment
    #[command(alias = "a")]
    Assignment {
        /// Course code (e.g., 02101)
        course_id: String,
        /// Assignment title
        title: String,
    },
    /// Compile a Typst file to PDF
    #[command(alias = "c")]
    Compile {
        /// Path to the .typ file (with or without extension)
        filepath: String,
    },
    /// Watch and auto-compile a Typst file
    #[command(alias = "w")]
    Watch {
        /// Path to the .typ file (with or without extension)
        filepath: String,
    },
    /// List recent notes for a course
    #[command(alias = "r")]
    Recent {
        /// Course code
        course_id: String,
    },
    /// Initialize repository structure
    Setup,
    /// Create Obsidian course index
    #[command(alias = "i")]
    Index {
        /// Course code
        course_id: String,
    },
    /// Search through notes
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,
    },
    /// Course management
    Courses {
        #[command(subcommand)]
        action: CourseAction,
    },
    
    /// Open most recent note for a course
    #[command(alias = "o")]
    Open {
        /// Course code
        course_id: String,
    },
    
    /// Show comprehensive status dashboard
    Status,
    
    /// Clean up compiled PDFs
    Clean,
    /// Show current semester info
    Semester,
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}


#[derive(Subcommand)]
pub enum CourseAction {
    /// List all courses
    List,
    /// Add a new course
    Add {
        /// Course code (e.g., 02101)
        course_id: String,
        /// Course name
        course_name: String,
    },
    /// Remove a course
    Remove {
        /// Course code to remove
        course_id: String,
    },
    /// Show common DTU course codes
    #[command(alias = "common")]
    Browse,
}



#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set author name
    SetAuthor {
        /// Author name
        name: String
    },
    /// Set preferred editor
    SetEditor {
        /// Editor command (e.g., code, nvim)
        editor: String
    },
    /// Reset configuration to defaults
    Reset,
    /// Show config file path
    Path,
    /// Validate current configuration
    Check,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::execute_command(&cli.command)?;
    Ok(())
}