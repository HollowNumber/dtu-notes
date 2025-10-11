//! # DTU Notes CLI
//!
//! A comprehensive command-line tool for managing notes and assignments at DTU
//! (Technical University of Denmark) with integrated Typst compilation and
//! Obsidian vault support.
//!
//! ## Features
//! - Dynamic template generation with automatic version detection
//! - Comprehensive project status monitoring and health analysis
//! - Seamless Typst compilation with file watching capabilities
//! - Obsidian vault integration for enhanced note management
//! - Course management with assignment tracking
//! - Extensible template system supporting custom repositories
//!
//! ## Usage Examples
//! ```bash
//! # Setup workspace
//! noter setup
//!
//! # Create lecture note
//! noter note 02101
//!
//! # Create assignment
//! noter assignment 02101 "Problem Set 1"
//!
//! # Compile to PDF
//! noter compile file.typ
//!
//! # Monitor system status
//! noter status
//! ```

mod commands;
mod config;
mod core;
mod data;
mod ui;

use anyhow::Result;
use clap::Parser;
use dtu_notes::{
    AssignmentAction, Commands, ConfigAction, CourseAction, SetupAction, TemplateAction,
};

#[cfg(feature = "dev-tools")]
use dtu_notes::DevAction;

/// Command-line interface structure using clap derive macros.
///
/// This structure defines the main CLI application with global configuration
/// and routing to subcommands.
#[derive(Parser)]
#[command(name = "noter")]
#[command(
    about = "A modern note-taking CLI for DTU students with Typst compilation and template management"
)]
#[command(long_about = "Noter - DTU Note-Taking CLI

A comprehensive command-line tool designed for students at Danmarks Tekniske Universitet (DTU)
to streamline note-taking, assignment management, and document compilation.

FEATURES:
  - Dynamic Typst Templates - Automatic template generation with course-specific variants
  - Smart Compilation - Integrated Typst compiler with watch mode and error handling
  - Obsidian Integration - Seamless sync with Obsidian vaults for enhanced note management
  - Course Management - Organize notes and assignments by course with automatic structure
  - Configuration System - Flexible config with interactive wizard and automatic migration
  - Template Repositories - Install and manage custom templates from GitHub
  - Health Monitoring - Track assignment progress and get actionable recommendations
  - Search & Discovery - Find notes and assignments across your entire workspace

QUICK START:
  noter setup              # Interactive setup wizard
  noter note 02101         # Create a lecture note for course 02101
  noter assignment 02101   # Create an assignment
  noter compile file.typ   # Compile Typst to PDF
  noter status             # View project health and statistics

CONFIGURATION:
  Use 'noter config interactive' for guided setup, or edit ~/.config/dtu-notes/config.json
  directly. See documentation at: https://github.com/HollowNumber/dtu-notes

For more information, visit: https://github.com/HollowNumber/dtu-notes")]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Main application entry point.
///
/// Parses command-line arguments using clap and routes execution to the
/// appropriate command handler. All errors are propagated using the `?` operator
/// and handled by anyhow's automatic error formatting.
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error with context
/// if any command fails.
fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::execute_command(&cli.command)?;
    Ok(())
}
