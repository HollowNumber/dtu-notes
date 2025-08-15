//! Template management commands
//!
//! Handles template downloads, updates, and status checking

use anyhow::Result;
use colored::Colorize;

use crate::config::get_config;
use crate::core::github_template_fetcher::GitHubTemplateFetcher;
use crate::ui::output::{OutputManager, Status};

/// Check template status and show version information
pub fn template_status() -> Result<()> {
    let config = get_config()?;

    OutputManager::print_section("Template Status", Some("📦"));

    // Check local template status
    let template_statuses = GitHubTemplateFetcher::check_template_status(&config)?;

    if template_statuses.is_empty() {
        OutputManager::print_status(Status::Warning, "No template repositories configured");
    } else {
        for (name, version) in &template_statuses {
            match version {
                Some(v) if v == "unknown" => {
                    OutputManager::print_status(
                        Status::Warning,
                        &format!("Template '{}' found but version unknown", name.yellow()),
                    );
                }
                Some(v) => {
                    OutputManager::print_status(
                        Status::Success,
                        &format!("Template '{}' installed: {}", name.yellow(), v.green()),
                    );
                }
                None => {
                    OutputManager::print_status(
                        Status::Error,
                        &format!("Template '{}' not found", name.yellow()),
                    );
                }
            }
        }
    }

    println!(
        "  Templates directory: {}",
        config.paths.templates_dir.dimmed()
    );
    println!(
        "  Typst packages directory: {}",
        config.paths.typst_packages_dir.dimmed()
    );

    // Check latest available versions for configured repositories
    println!("\n{} Checking latest versions...", "🌐".blue());

    // Check custom repositories
    for repo_config in &config.templates.custom_repositories {
        if !repo_config.enabled {
            continue;
        }

        match GitHubTemplateFetcher::get_latest_release(&repo_config.repository) {
            Ok(release) => {
                println!(
                    "  {} ({}): {}",
                    repo_config.name.yellow(),
                    repo_config.repository.dimmed(),
                    release.tag_name.green()
                );
            }
            Err(e) => {
                println!(
                    "  {} ({}): {}",
                    repo_config.name.yellow(),
                    repo_config.repository.dimmed(),
                    format!("Error: {}", e).red()
                );
            }
        }
    }

    // Check official if fallback is enabled
    if config.templates.use_official_fallback {
        match GitHubTemplateFetcher::get_latest_release("HollowNumber/dtu-note-template") {
            Ok(release) => {
                println!(
                    "  {}: {}",
                    "dtu_template (fallback)".blue(),
                    release.tag_name.green()
                );
            }
            Err(e) => {
                println!(
                    "  {}: {}",
                    "dtu_template (fallback)".blue(),
                    format!("Error: {}", e).red()
                );
            }
        }
    }

    println!();
    OutputManager::print_command_examples(&[
        ("noter template update", "Update to latest versions"),
        ("noter template reinstall", "Force reinstall templates"),
        (
            "noter config add-template-repo <name> <owner/repo>",
            "Add custom template",
        ),
        (
            "noter config list-template-repos",
            "List template repositories",
        ),
    ]);

    Ok(())
}

/// Update templates to the latest version
pub fn update_template() -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(Status::Loading, "Checking for template updates...");

    // Get current versions (for potential future use)
    let _current_statuses = GitHubTemplateFetcher::check_template_status(&config)?;

    // Update templates
    let results = GitHubTemplateFetcher::update_templates(&config)?;

    if results.is_empty() {
        OutputManager::print_status(
            Status::Warning,
            "No templates were updated (no repositories configured?)",
        );
        return Ok(());
    }

    for result in results {
        OutputManager::print_status(
            Status::Success,
            &format!(
                "Updated template: {} -> {}",
                result
                    .installed_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("template"),
                result.version.green()
            ),
        );

        println!(
            "Templates installed at: {}",
            result.installed_path.display().to_string().dimmed()
        );
    }

    Ok(())
}

/// Force reinstall templates (useful for fixing corrupted installations)
pub fn reinstall_template() -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(Status::Loading, "Reinstalling templates...");

    let results = GitHubTemplateFetcher::download_and_install_templates(&config, true)?;

    if results.is_empty() {
        OutputManager::print_status(
            Status::Warning,
            "No templates were installed (no repositories configured?)",
        );
        return Ok(());
    }

    for result in results {
        OutputManager::print_status(
            Status::Success,
            &format!(
                "Reinstalled template: {} ({})",
                result
                    .installed_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("template"),
                result.version.green()
            ),
        );

        println!(
            "Templates installed at: {}",
            result.installed_path.display().to_string().dimmed()
        );

        if !result.is_cached {
            println!("Downloaded fresh copy from GitHub");
        }
    }

    Ok(())
}

/// Create a custom template using the TemplateBuilder
pub fn create_custom_template(
    course_id: &str,
    title: &str,
    template_type: &str,
    sections: Option<&str>,
) -> Result<()> {
    use crate::core::template_engine::{TemplateBuilder, TemplateType};
    use crate::core::validation::Validator;
    use std::fs;
    use std::path::Path;

    let config = get_config()?;

    // Validate course ID
    Validator::validate_course_id(course_id)?;

    // Parse template type
    let t_type = match template_type.to_lowercase().as_str() {
        "lecture" | "l" => TemplateType::Lecture,
        "assignment" | "a" => TemplateType::Assignment,
        custom => TemplateType::Custom(custom.to_string()),
    };

    OutputManager::print_status(
        Status::Loading,
        &format!(
            "Creating custom {} template for {}",
            template_type,
            course_id.yellow()
        ),
    );

    // Build template with TemplateBuilder
    let mut builder = TemplateBuilder::new(course_id, &config)?
        .with_title(title)
        .with_type(t_type);

    // Parse custom sections if provided
    if let Some(sections_str) = sections {
        let custom_sections: Vec<String> = sections_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !custom_sections.is_empty() {
            builder = builder.with_sections(custom_sections);
        }
    } else {
        // Use default sections based on type
        let default_sections = match template_type.to_lowercase().as_str() {
            "assignment" => config.note_preferences.assignment_sections.clone(),
            _ => config.note_preferences.lecture_sections.clone(),
        };
        builder = builder.with_sections(default_sections);
    }

    // Generate template content and filename
    let (content, filename) = builder.build_with_filename()?;

    // Create output directory
    let output_dir = Path::new(&config.paths.notes_dir)
        .join(course_id)
        .join("custom-templates");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    // Write template file
    let file_path = output_dir.join(&filename);
    fs::write(&file_path, &content)?;

    OutputManager::print_status(
        Status::Success,
        &format!(
            "Custom template created: {}",
            file_path.display().to_string().bright_white()
        ),
    );

    // Auto-open if configured
    if config.note_preferences.auto_open {
        OutputManager::print_status(Status::Info, "Opening in editor...");

        if let Some(editor) = &config.preferred_editor {
            if let Err(_) = std::process::Command::new(editor).arg(&file_path).spawn() {
                // Fallback to system default
                let _ = opener::open(&file_path);
            }
        } else {
            let _ = opener::open(&file_path);
        }

        OutputManager::print_status(Status::Success, "Opened file with system default");
    }

    // Show what was created
    println!();
    println!("Template details:");
    println!("  Type: {}", template_type.bright_blue());
    println!(
        "  Course: {} - {}",
        course_id.yellow(),
        config
            .courses
            .get(course_id)
            .unwrap_or(&"Unknown Course".to_string())
            .dimmed()
    );
    println!("  Title: {}", title.green());

    // Show sections if any
    let sections_used = if let Some(s) = sections {
        s.split(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        match template_type.to_lowercase().as_str() {
            "assignment" => config.note_preferences.assignment_sections.join(", "),
            _ => config.note_preferences.lecture_sections.join(", "),
        }
    };

    if !sections_used.is_empty() {
        println!("  Sections: {}", sections_used.dimmed());
    }

    println!();
    OutputManager::print_command_examples(&[
        (
            &format!("noter compile {}", file_path.display()),
            "Compile to PDF",
        ),
        (
            &format!("noter watch {}", file_path.display()),
            "Watch and auto-compile",
        ),
    ]);

    Ok(())
}
