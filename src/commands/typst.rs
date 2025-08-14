use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn compile_file(filepath: &str) -> Result<()> {
    let filepath = normalize_filepath(filepath);

    if !Path::new(&filepath).exists() {
        println!("{} File not found: {}", "❌".red(), filepath.yellow());
        println!("Make sure the file exists and try again.");
        return Ok(());
    }

    println!("{} Compiling {}...", "🔨".blue(), filepath.bright_white());

    let output = Command::new("typst")
        .args(&["compile", &filepath])
        .output()?;

    if output.status.success() {
        println!("{} Compiled successfully!", "✅".green());
    } else {
        println!("{} Compilation failed:", "❌".red());
        println!("{}", String::from_utf8_lossy(&output.stderr).bright_red());
    }

    Ok(())
}


pub fn watch_file(filepath: &str) -> Result<()> {
    let filepath = normalize_filepath(filepath);

    if !Path::new(&filepath).exists() {
        println!("{} File not found: {}", "❌".red(), filepath);
        return Ok(());
    }

    println!("{} Watching {} for changes...", "👀".blue(), filepath);

    let mut child = Command::new("typst")
        .args(&["watch", &filepath])
        .spawn()?;

    child.wait()?;
    Ok(())
}

pub fn clean_files() -> Result<()> {
    let config = crate::config::get_config()?;
    println!("{} Cleaning up compiled files...", "🧹".blue());

    if Path::new(&config.paths.notes_dir).exists() {
        clean_directory(&config.paths.notes_dir)?;
    } else {
        println!("{} Notes directory not found: {}", "⚠️".yellow(), config.paths.notes_dir);
    }

    println!("{} Cleanup complete!", "✅".green());
    Ok(())
}


fn clean_directory(dir: &str) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                clean_directory(path_str)?;
            }
        } else if path.extension().map_or(false, |ext| ext == "pdf") {
            fs::remove_file(&path)?;
            println!("Removed: {}", path.display());
        }
    }
    Ok(())
}

fn normalize_filepath(filepath: &str) -> String {
    if filepath.ends_with(".typ") {
        filepath.to_string()
    } else {
        format!("{}.typ", filepath)
    }
}