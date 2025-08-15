//! Validation utilities
//!
//! Centralized validation logic for various input types.

use anyhow::Result;

pub struct Validator;

#[allow(dead_code)]
impl Validator {
    pub fn validate_course_id(course_id: &str) -> Result<()> {
        if course_id.len() != 5 {
            anyhow::bail!("Course ID must be exactly 5 characters long (e.g., 02101)");
        }

        if !course_id.chars().all(|c| c.is_ascii_digit()) {
            anyhow::bail!("Course ID must contain only digits (e.g., 02101)");
        }

        Ok(())
    }

    pub fn sanitize_filename(input: &str) -> String {
        input
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
                'æ' | 'Æ' => 'a',
                'ø' | 'Ø' => 'o',
                'å' | 'Å' => 'a',
                'ä' | 'Ä' => 'a',
                'ö' | 'Ö' => 'o',
                ' ' | '.' | ',' | ';' | ':' | '/' | '\\' => '-',
                _ => '-',
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
            .to_lowercase()
            .trim_end_matches('-')
            .to_string()
    }

    pub fn validate_file_path(path: &str) -> Result<()> {
        if path.is_empty() {
            anyhow::bail!("File path cannot be empty");
        }

        // Add more path validation as needed
        Ok(())
    }
}