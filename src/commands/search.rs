use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::get_config;

pub fn search_notes(query: &str) -> Result<()> {
    let config = get_config()?;

    println!("{} Searching for '{}'...", "🔍".blue(), query);

    if !Path::new(&config.paths.notes_dir).exists() {
        println!("No notes directory found at: {}", config.paths.notes_dir);
        return Ok(());
    }

    let mut results = Vec::new();
    search_in_directory(&config.paths.notes_dir, query, &config, &mut results)?;

    if results.is_empty() {
        println!("No results found");
        return Ok(());
    }

    // Limit results and show them
    results.truncate(config.search.max_results);

    for (path, line_num, line) in results {
        let highlighted = if config.search.case_sensitive {
            line.replace(query, &format!("{}", query.bright_yellow()))
        } else {
            let lower_query = query.to_lowercase();
            let lower_line = line.to_lowercase();
            if let Some(pos) = lower_line.find(&lower_query) {
                let mut result = line.clone();
                let actual_match = &line[pos..pos + query.len()];
                result = result.replace(actual_match, &format!("{}", actual_match.bright_yellow()));
                result
            } else {
                line
            }
        };

        println!("{}:{}: {}", path.display().to_string().bright_blue(), line_num.to_string().dimmed(), highlighted);
    }

    Ok(())
}

fn search_in_directory(
    dir: &str,
    query: &str,
    config: &crate::config::Config,
    results: &mut Vec<(std::path::PathBuf, usize, String)>
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                search_in_directory(path_str, query, config, results)?;
            }
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if config.search.file_extensions.contains(&ext_str) {
                if let Ok(content) = fs::read_to_string(&path) {
                    for (line_num, line) in content.lines().enumerate() {
                        let matches = if config.search.case_sensitive {
                            line.contains(query)
                        } else {
                            line.to_lowercase().contains(&query.to_lowercase())
                        };

                        if matches {
                            results.push((path.clone(), line_num + 1, line.trim().to_string()));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
