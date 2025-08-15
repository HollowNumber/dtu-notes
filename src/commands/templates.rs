//! Template management commands
//!
//! Handles template downloads, updates, and status checking

use anyhow::Result;
use colored::Colorize;

use crate::config::get_config;
use crate::core::github_template_fetcher::{GitHubTemplateFetcher, GitHubRelease};
use crate::ui::output::{OutputManager, Status};

/// Check template status and show version information
pub fn template_status() -> Result<()> {
    let config = get_config()?;
    
    OutputManager::print_section("Template Status", Some("📦"));
    
    // Check local template status
    let template_statuses = GitHubTemplateFetcher::check_template_status(&config)?;
    
    if template_statuses.is_empty() {
        OutputManager::print_status(
            Status::Warning,
            "No template repositories configured"
        );
    } else {
        for (name, version) in &template_statuses {
            match version {
                Some(v) if v == "unknown" => {
                    OutputManager::print_status(
                        Status::Warning,
                        &format!("Template '{}' found but version unknown", name.yellow())
                    );
                }
                Some(v) => {
                    OutputManager::print_status(
                        Status::Success,
                        &format!("Template '{}' installed: {}", name.yellow(), v.green())
                    );
                }
                None => {
                    OutputManager::print_status(
                        Status::Error,
                        &format!("Template '{}' not found", name.yellow())
                    );
                }
            }
        }
    }
    
    println!("  Templates directory: {}", config.paths.templates_dir.dimmed());
    println!("  Typst packages directory: {}", config.paths.typst_packages_dir.dimmed());
    
    // Check latest available versions for configured repositories
    println!("\n{} Checking latest versions...", "🌐".blue());
    
    // Check custom repositories
    for repo_config in &config.templates.custom_repositories {
        if !repo_config.enabled {
            continue;
        }
        
        match GitHubTemplateFetcher::get_latest_release(&repo_config.repository) {
            Ok(release) => {
                println!("  {} ({}): {}", 
                         repo_config.name.yellow(), 
                         repo_config.repository.dimmed(),
                         release.tag_name.green());
            }
            Err(e) => {
                println!("  {} ({}): {}", 
                         repo_config.name.yellow(), 
                         repo_config.repository.dimmed(),
                         format!("Error: {}", e).red());
            }
        }
    }
    
    // Check official if fallback is enabled
    if config.templates.use_official_fallback {
        match GitHubTemplateFetcher::get_latest_release("HollowNumber/dtu-note-template") {
            Ok(release) => {
                println!("  {}: {}", 
                         "dtu_template (fallback)".blue(), 
                         release.tag_name.green());
            }
            Err(e) => {
                println!("  {}: {}", 
                         "dtu_template (fallback)".blue(), 
                         format!("Error: {}", e).red());
            }
        }
    }
    
    println!();
    OutputManager::print_command_examples(&[
        ("noter template update", "Update to latest versions"),
        ("noter template reinstall", "Force reinstall templates"),
        ("noter config add-template-repo <name> <owner/repo>", "Add custom template"),
        ("noter config list-template-repos", "List template repositories"),
    ]);
    
    Ok(())
}

/// Update templates to the latest version
pub fn update_template() -> Result<()> {
    let config = get_config()?;
    
    OutputManager::print_status(Status::Loading, "Checking for template updates...");
    
    // Get current versions
    let current_statuses = GitHubTemplateFetcher::check_template_status(&config)?;
    
    // Update templates
    let results = GitHubTemplateFetcher::update_templates(&config)?;
    
    if results.is_empty() {
        OutputManager::print_status(
            Status::Warning, 
            "No templates were updated (no repositories configured?)"
        );
        return Ok(());
    }
    
    for result in results {
        OutputManager::print_status(
            Status::Success,
            &format!("Updated template: {} -> {}", 
                     result.installed_path.file_name()
                         .and_then(|n| n.to_str())
                         .unwrap_or("template"), 
                     result.version.green())
        );
        
        println!("Templates installed at: {}", result.installed_path.display().to_string().dimmed());
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
            "No templates were installed (no repositories configured?)"
        );
        return Ok(());
    }
    
    for result in results {
        OutputManager::print_status(
            Status::Success,
            &format!("Reinstalled template: {} ({})", 
                     result.installed_path.file_name()
                         .and_then(|n| n.to_str())
                         .unwrap_or("template"), 
                     result.version.green())
        );
        
        println!("Templates installed at: {}", result.installed_path.display().to_string().dimmed());
        
        if !result.is_cached {
            println!("Downloaded fresh copy from GitHub");
        }
    }
    
    Ok(())
}
