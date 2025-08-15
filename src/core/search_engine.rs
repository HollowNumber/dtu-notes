//! Search engine for note content
//!
//! Handles searching through files with various options and filters.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub max_results: usize,
    pub context_lines: usize,
    pub file_extensions: Vec<String>,
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn search_in_directory<P: AsRef<Path>>(
        dir: P,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchMatch>> {
        let mut results = Vec::new();
        Self::search_recursive(dir.as_ref(), query, options, &mut results)?;

        // Limit results
        results.truncate(options.max_results);
        Ok(results)
    }

    fn search_recursive(
        dir: &Path,
        query: &str,
        options: &SearchOptions,
        results: &mut Vec<SearchMatch>,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::search_recursive(&path, query, options, results)?;
            } else if Self::should_search_file(&path, options) {
                Self::search_in_file(&path, query, options, results)?;
            }
        }
        Ok(())
    }

    fn should_search_file(path: &Path, options: &SearchOptions) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            options.file_extensions.contains(&ext_str)
        } else {
            false
        }
    }

    fn search_in_file(
        path: &Path,
        query: &str,
        options: &SearchOptions,
        results: &mut Vec<SearchMatch>,
    ) -> Result<()> {
        let content = fs::read_to_string(path)?;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(match_pos) = Self::find_match(line, query, options.case_sensitive) {
                results.push(SearchMatch {
                    file_path: path.to_path_buf(),
                    line_number: line_num + 1,
                    line_content: line.trim().to_string(),
                    match_start: match_pos,
                    match_end: match_pos + query.len(),
                });
            }
        }

        Ok(())
    }

    fn find_match(line: &str, query: &str, case_sensitive: bool) -> Option<usize> {
        if case_sensitive {
            line.find(query)
        } else {
            line.to_lowercase().find(&query.to_lowercase())
        }
    }
}
