//! Typst compilation commands
//!
//! Thin command layer that delegates to core typst compiler.

use anyhow::Result;
use colored::Colorize;

use crate::config::get_config;
use crate::core::typst_compiler::TypstCompiler;
use crate::ui::output::{OutputManager, Status};

pub fn compile_file(filepath: &str) -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(
        Status::Loading,
        &format!("Compiling {}", filepath.bright_white())
    );

    match TypstCompiler::compile_file(filepath, &config) {
        Ok(output_path) => {
            OutputManager::print_status(
                Status::Success,
                &format!("Compiled successfully: {}", output_path.bright_green())
            );

            // Show file size if available
            if let Ok(metadata) = std::fs::metadata(&output_path) {
                let size_kb = metadata.len() / 1024;
                println!("File size: {} KB", size_kb.to_string().dimmed());
            }

            // Auto-open the compiled PDF if configured to do so
            if config.note_preferences.auto_open {
                OutputManager::print_status(Status::Info, "Opening compiled PDF...");
                if let Err(e) = opener::open(&output_path) {
                    OutputManager::print_status(
                        Status::Warning,
                        &format!("Could not open PDF automatically: {}", e)
                    );
                }
            } else {
                println!("PDF created at: {}", output_path);
            }

            // Show helpful next steps
            OutputManager::print_command_examples(&[
                (&format!("noter watch {}", filepath), "Watch for changes"),
                (&format!("opener {}", output_path), "Open PDF manually"),
            ]);
        }
        Err(e) => {
            OutputManager::print_status(Status::Error, &format!("Compilation failed: {}", e));

            if e.to_string().contains("not found") {
                println!("Make sure Typst is installed: {}",
                         "https://github.com/typst/typst#installation".bright_blue());
            }
        }
    }

    Ok(())
}


pub fn watch_file(filepath: &str) -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(
        Status::Info,
        &format!("Watching {} for changes...", filepath.bright_white())
    );

    println!("Press {} to stop", "Ctrl+C".yellow());

    match TypstCompiler::watch_file(filepath, &config) {
        Ok(_) => {
            OutputManager::print_status(Status::Info, "Watch stopped");
        }
        Err(e) => {
            OutputManager::print_status(Status::Error, &format!("Watch failed: {}", e));
        }
    }

    Ok(())
}

pub fn clean_files() -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(Status::Loading, "Cleaning compiled files...");

    match TypstCompiler::clean_files(&config) {
        Ok(cleaned_count) => {
            if cleaned_count > 0 {
                OutputManager::print_status(
                    Status::Success,
                    &format!("Cleaned {} PDF files", cleaned_count)
                );
            } else {
                OutputManager::print_status(Status::Info, "No PDF files found to clean");
            }
        }
        Err(e) => {
            OutputManager::print_status(Status::Error, &format!("Clean failed: {}", e));
        }
    }

    Ok(())
}