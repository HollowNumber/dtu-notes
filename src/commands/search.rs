//! Search command implementation
//!
//! Thin command layer that uses core search engine and ui formatters.

use anyhow::Result;
use std::path::Path;

use crate::config::get_config;
use crate::core::search_engine::{SearchEngine, SearchOptions};
use crate::ui::formatters::Formatters;
use crate::ui::output::{OutputManager, Status};

pub fn search_notes(query: &str) -> Result<()> {
    let config = get_config()?;

    OutputManager::print_status(Status::Loading, &format!("Searching for '{}'", query));

    if !Path::new(&config.paths.notes_dir).exists() {
        OutputManager::print_status(
            Status::Warning,
            &format!("No notes directory found at: {}", config.paths.notes_dir)
        );
        return Ok(());
    }

    let search_options = SearchOptions {
        case_sensitive: config.search.case_sensitive,
        max_results: config.search.max_results,
        context_lines: config.search.context_lines,
        file_extensions: config.search.file_extensions,
    };

    let results = SearchEngine::search_in_directory(
        &config.paths.notes_dir,
        query,
        &search_options,
    )?;

    if results.is_empty() {
        OutputManager::print_status(Status::Info, "No results found");
    } else {
        let formatted_results = Formatters::format_search_results(&results, query);
        println!("{}", formatted_results);

        if results.len() >= config.search.max_results {
            OutputManager::print_status(
                Status::Info,
                &format!("Showing first {} results (limit reached)", config.search.max_results)
            );
        }
    }

    Ok(())
}