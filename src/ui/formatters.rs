//! Output formatting utilities
//!
//! Consistent formatting for different types of output.

use crate::core::search_engine::SearchMatch;
use colored::*;

pub struct Formatters;

#[allow(dead_code)]
impl Formatters {
    pub fn format_course_list(courses: &[(String, String)]) -> String {
        if courses.is_empty() {
            return format!("{} No courses configured.", "ℹ️".blue());
        }

        let mut output = format!("{} Your DTU Courses:\n\n", "🎓".blue());

        for (course_id, course_name) in courses {
            output.push_str(&format!("  {} - {}\n", course_id.yellow(), course_name));
        }

        output.push('\n');
        output.push_str(&format!(
            "{} Total: {} courses\n",
            "📊".blue(),
            courses.len().to_string().green()
        ));

        output
    }

    pub fn format_search_results(results: &[SearchMatch], query: &str) -> String {
        if results.is_empty() {
            return "No results found".to_string();
        }

        let mut output = format!(
            "{} Search Results for '{}':\n\n",
            "🔍".blue(),
            query.bright_white()
        );

        for result in results {
            let highlighted = Self::highlight_precise_match(
                &result.line_content,
                result.match_start,
                result.match_end,
            );
            output.push_str(&format!(
                "{}:{}: {}\n",
                result.file_path.display().to_string().bright_blue(),
                result.line_number.to_string().dimmed(),
                highlighted
            ));
        }

        output.push_str(&format!(
            "\n{} {} results found\n",
            "📊".blue(),
            results.len().to_string().green()
        ));
        output
    }

    pub fn format_status_section(title: &str, icon: &str, content: &str) -> String {
        format!("{} {}:\n{}\n", icon.blue(), title, content)
    }

    pub fn format_success(message: &str) -> String {
        format!("{} {}", "✅".green(), message)
    }

    pub fn format_warning(message: &str) -> String {
        format!("{} {}", "⚠️".yellow(), message)
    }

    pub fn format_error(message: &str) -> String {
        format!("{} {}", "❌".red(), message)
    }

    pub fn format_info(message: &str) -> String {
        format!("{} {}", "ℹ️".blue(), message)
    }

    fn highlight_match(line: &str, query: &str) -> String {
        // Case-insensitive highlighting
        let lower_line = line.to_lowercase();
        let lower_query = query.to_lowercase();

        if let Some(pos) = lower_line.find(&lower_query) {
            let result = line.to_string();
            let actual_match = &line[pos..pos + query.len()];
            result.replace(actual_match, &format!("{}", actual_match.bright_yellow()))
        } else {
            line.to_string()
        }
    }

    fn highlight_precise_match(line: &str, match_start: usize, match_end: usize) -> String {
        if match_start < line.len() && match_end <= line.len() && match_start < match_end {
            let before = &line[..match_start];
            let matched = &line[match_start..match_end];
            let after = &line[match_end..];
            format!("{}{}{}", before, matched.bright_yellow().bold(), after)
        } else {
            // Fallback to basic highlighting
            line.to_string()
        }
    }
}
