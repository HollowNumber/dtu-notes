use anyhow::Result;
use colored::*;

use crate::config::{get_config, update_author, update_editor, Config, TemplateRepository};

pub fn show_config() -> Result<()> {
    let config = get_config()?;

    println!("{} Current Configuration:", "⚙️".blue());
    println!();
    println!("Author: {}", config.author.green());
    println!("Preferred Editor: {}",
             config.preferred_editor.as_deref().unwrap_or("None").yellow());
    println!("Template Version: {}", config.template_version);
    println!("Semester Format: {:?}", config.semester_format);
    println!();
    println!("Paths:");
    println!("  Notes: {}", config.paths.notes_dir);
    println!("  Obsidian: {}", config.paths.obsidian_dir);
    println!("  Templates: {}", config.paths.templates_dir);
    println!("  Typst Packages: {}", config.paths.typst_packages_dir);
    println!();
    println!("Preferences:");
    println!("  Auto Open: {}", config.note_preferences.auto_open);
    println!("  Include Date in Title: {}", config.note_preferences.include_date_in_title);
    println!("  Create Backups: {}", config.note_preferences.create_backups);
    println!();
    println!("Search:");
    println!("  Max Results: {}", config.search.max_results);
    println!("  Case Sensitive: {}", config.search.case_sensitive);
    println!("  File Extensions: {}", config.search.file_extensions.join(", "));
    println!();
    println!("Templates:");
    println!("  Use Official Fallback: {}", config.templates.use_official_fallback);
    println!("  Auto Update: {}", config.templates.auto_update);
    println!("  Enable Caching: {}", config.templates.enable_caching);
    
    if config.templates.custom_repositories.is_empty() {
        println!("  Custom Repositories: None");
    } else {
        println!("  Custom Repositories:");
        for repo in &config.templates.custom_repositories {
            let status = if repo.enabled { "✅" } else { "❌" };
            println!("    {} {} ({})", status, repo.name.green(), repo.repository.yellow());
            if let Some(version) = &repo.version {
                println!("      Version: {}", version);
            }
            if let Some(branch) = &repo.branch {
                println!("      Branch: {}", branch);
            }
            if let Some(path) = &repo.template_path {
                println!("      Template Path: {}", path);
            }
        }
    }

    Ok(())
}

pub fn set_author(name: &str) -> Result<()> {
    update_author(name.to_string())?;
    println!("{} Author updated to: {}", "✅".green(), name.green());
    Ok(())
}

pub fn set_editor(editor: &str) -> Result<()> {
    update_editor(Some(editor.to_string()))?;
    println!("{} Preferred editor set to: {}", "✅".green(), editor.yellow());
    Ok(())
}

pub fn add_template_repository(
    name: &str,
    repository: &str,
    version: Option<&str>,
    template_path: Option<&str>,
) -> Result<()> {
    let mut config = get_config()?;
    
    // Check if repository already exists
    if config.templates.custom_repositories.iter().any(|r| r.name == name) {
        return Err(anyhow::anyhow!("Template repository '{}' already exists", name));
    }

    let template_repo = TemplateRepository {
        name: name.to_string(),
        repository: repository.to_string(),
        version: version.map(|v| v.to_string()),
        branch: None,
        template_path: template_path.map(|p| p.to_string()),
        enabled: true,
    };

    config.templates.custom_repositories.push(template_repo);
    config.save()?;

    println!("{} Added template repository: {} ({})", 
             "✅".green(), name.green(), repository.yellow());
    Ok(())
}

pub fn remove_template_repository(name: &str) -> Result<()> {
    let mut config = get_config()?;
    
    let initial_len = config.templates.custom_repositories.len();
    config.templates.custom_repositories.retain(|r| r.name != name);
    
    if config.templates.custom_repositories.len() == initial_len {
        return Err(anyhow::anyhow!("Template repository '{}' not found", name));
    }

    config.save()?;
    println!("{} Removed template repository: {}", "🗑️".red(), name);
    Ok(())
}

pub fn enable_template_repository(name: &str, enabled: bool) -> Result<()> {
    let mut config = get_config()?;
    
    let repo = config.templates.custom_repositories
        .iter_mut()
        .find(|r| r.name == name)
        .ok_or_else(|| anyhow::anyhow!("Template repository '{}' not found", name))?;
    
    repo.enabled = enabled;
    config.save()?;
    
    let status = if enabled { "enabled" } else { "disabled" };
    let emoji = if enabled { "✅" } else { "❌" };
    println!("{} Template repository '{}' {}", emoji, name, status);
    Ok(())
}

pub fn list_template_repositories() -> Result<()> {
    let config = get_config()?;
    
    if config.templates.custom_repositories.is_empty() {
        println!("{} No custom template repositories configured", "📝".blue());
        println!("Add one with: {}", 
                 "noter config add-template-repo <name> <owner/repo>".bright_white());
    } else {
        println!("{} Template Repositories:", "📦".blue());
        for repo in &config.templates.custom_repositories {
            let status = if repo.enabled { "✅" } else { "❌" };
            println!("  {} {} ({})", status, repo.name.green(), repo.repository.yellow());
            if let Some(version) = &repo.version {
                println!("    Version: {}", version);
            }
            if let Some(path) = &repo.template_path {
                println!("    Template Path: {}", path);
            }
        }
    }
    
    if config.templates.use_official_fallback {
        println!("  {} official (fallback)", "🏛️".blue());
    }
    
    Ok(())
}

pub fn set_template_auto_update(enabled: bool) -> Result<()> {
    let mut config = get_config()?;
    config.templates.auto_update = enabled;
    config.save()?;
    
    let status = if enabled { "enabled" } else { "disabled" };
    println!("{} Template auto-update {}", "🔄".blue(), status);
    Ok(())
}

pub fn reset_config() -> Result<()> {
    let default_config = Config::default();
    default_config.save()?;
    println!("{} Configuration reset to defaults", "🔄".blue());
    Ok(())
}

pub fn show_config_path() -> Result<()> {
    let path = Config::config_file_path()?;
    println!("{} Config file location:", "📁".blue());
    println!("{}", path.display());
    Ok(())
}

pub fn check_config() -> Result<()> {
    let config = get_config()?;
    let warnings = config.validate()?;

    if warnings.is_empty() {
        println!("{} Configuration is valid!", "✅".green());
    } else {
        println!("{} Configuration warnings:", "⚠️".yellow());
        for warning in warnings {
            println!("  • {}", warning);
        }
    }

    Ok(())
}
