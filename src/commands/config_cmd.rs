use anyhow::Result;
use colored::*;

use crate::config::{get_config, update_author, update_editor, Config};

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
