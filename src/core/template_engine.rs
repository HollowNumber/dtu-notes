//! Template engine for generating note and assignment files
//!
//! Handles template generation, content formatting, and file creation
//! with configurable sections and metadata.

use anyhow::Result;
use chrono::Local;
use std::collections::HashMap;
use std::path::Path;

use crate::config::Config;
use crate::core::status_manager::StatusManager;
use crate::core::validation::Validator;
use crate::core::github_template_fetcher::GitHubTemplateFetcher;

#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub semester: String,
    pub template_version: String,
    pub sections: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum TemplateType {
    Lecture,
    Assignment,
    Custom(String),
}

pub struct TemplateEngine;

impl TemplateEngine {
    /// Ensure templates are available, download if necessary
    pub fn ensure_templates_available(config: &Config) -> Result<()> {
        // Check if templates are already available
        let template_statuses = GitHubTemplateFetcher::check_template_status(config)?;
        
        // Check if we have any installed templates
        let has_templates = template_statuses.iter().any(|(_, version)| version.is_some());
        
        if !has_templates {
            // No templates found, download from configured repositories
            let _download_results = GitHubTemplateFetcher::download_and_install_templates(config, false)?;
        }
        
        Ok(())
    }

    /// Generate a lecture note template
    pub fn generate_lecture_template(
        course_id: &str,
        config: &Config,
        custom_title: Option<&str>,
    ) -> Result<String> {
        let context = Self::build_lecture_context(course_id, config, custom_title)?;
        Self::render_template(&context, &TemplateType::Lecture)
    }

    /// Generate an assignment template
    pub fn generate_assignment_template(
        course_id: &str,
        assignment_title: &str,
        config: &Config,
    ) -> Result<String> {
        let context = Self::build_assignment_context(course_id, assignment_title, config)?;
        Self::render_template(&context, &TemplateType::Assignment)
    }

    /// Build lecture context
    fn build_lecture_context(
        course_id: &str,
        config: &Config,
        custom_title: Option<&str>,
    ) -> Result<TemplateContext> {
        let course_name = Self::resolve_course_name(course_id, config);
        let semester = StatusManager::get_current_semester(config);

        let title = if let Some(custom_title) = custom_title {
            custom_title.to_string()
        } else {
            let date = chrono::Local::now();
            if config.note_preferences.include_date_in_title {
                format!("Lecture - {}", date.format("%B %d, %Y"))
            } else {
                "Lecture Notes".to_string()
            }
        };

        Ok(TemplateContext {
            course_id: course_id.to_string(),
            course_name,
            title,
            author: config.author.clone(),
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            semester,
            template_version: config.template_version.clone(),
            sections: config.note_preferences.lecture_sections.clone(),
            custom_fields: HashMap::new(),
        })
    }

    /// Build assignment context
    fn build_assignment_context(
        course_id: &str,
        assignment_title: &str,
        config: &Config,
    ) -> Result<TemplateContext> {
        let course_name = Self::resolve_course_name(course_id, config);
        let semester = StatusManager::get_current_semester(config);

        Ok(TemplateContext {
            course_id: course_id.to_string(),
            course_name,
            title: assignment_title.to_string(),
            author: config.author.clone(),
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            semester,
            template_version: config.template_version.clone(),
            sections: config.note_preferences.assignment_sections.clone(),
            custom_fields: HashMap::new(),
        })
    }


    /// Render the template with the given context
    fn render_template(context: &TemplateContext, template_type: &TemplateType) -> Result<String> {
        let header = Self::generate_typst_header(context, template_type)?;
        let sections = Self::generate_sections(&context.sections, template_type)?;

        Ok(format!("{}\n{}", header, sections))
    }


    /// Generate the Typst document header
    fn generate_typst_header(context: &TemplateContext, template_type: &TemplateType) -> Result<String> {
        let template_name = match template_type {
            TemplateType::Lecture => "dtu-note",
            TemplateType::Assignment => "dtu-assignment",
            TemplateType::Custom(template) => template,
        };

        // For assignments, use due-date instead of date
        let date_param = match template_type {
            TemplateType::Assignment => "due-date",
            _ => "date",
        };

        // Use course name from config, or fall back to course_id if not available
        let course_name = if context.course_name.is_empty() {
            &context.course_id
        } else {
            &context.course_name
        };

        Ok(format!(
            r#"#import "@local/dtu-template:{}": *

#show: {}.with(
  course: "{}",
  course-name: "{}",
  title: "{}",
  {}: datetime.today(),
  author: "{}",
  semester: "{}"
)"#,
            context.template_version,
            template_name,
            context.course_id,
            course_name,
            context.title,
            date_param,
            context.author,
            context.semester,
        ))
    }




    /// Generate section content based on template type
    fn generate_sections(sections: &[String], template_type: &TemplateType) -> Result<String> {
        let mut content = String::new();

        for section in sections {
            content.push_str(&format!("\n= {}\n", section));

            // Add type-specific content for certain sections
            match template_type {
                TemplateType::Lecture => {
                    content.push_str(&Self::generate_lecture_section_content(section));
                }
                TemplateType::Assignment => {
                    content.push_str(&Self::generate_assignment_section_content(section));
                }
                TemplateType::Custom(_) => {
                    content.push_str("\n\n");
                }
            }
        }

        Ok(content)
    }

    /// Generate content for lecture-specific sections
    fn generate_lecture_section_content(section: &str) -> String {
        match section {
            "Examples" => "\n#example[\n  Insert example here...\n]\n".to_string(),
            "Important Points" => "\n#important[\n  Key takeaways from today's lecture\n]\n".to_string(),
            "Questions" => "\n#question[\n  What questions do I have about this topic?\n]\n".to_string(),
            "Summary" => "\n#summary[\n  Brief summary of the main concepts\n]\n".to_string(),
            _ => "\n\n".to_string(),
        }
    }

    /// Generate content for assignment-specific sections
    fn generate_assignment_section_content(section: &str) -> String {
        match section {
            "Problem Statement" => "\n#problem[\n  State the problem clearly\n]\n".to_string(),
            "Solution" => "\n#solution[\n  Step-by-step solution\n]\n".to_string(),
            "Analysis" => "\n#analysis[\n  Analysis of the results\n]\n".to_string(),
            "Conclusion" => "\n#conclusion[\n  Final conclusions and insights\n]\n".to_string(),
            "Code" => "\n```python\n# Insert code here\n```\n".to_string(),
            "Calculations" => "\n$ \"Insert mathematical calculations here\" $\n".to_string(),
            _ => "\n\n".to_string(),
        }
    }

    /// Resolve course name from config
    fn resolve_course_name(course_id: &str, config: &Config) -> String {
        if let Some(course_name) = config.courses.get(course_id) {
            course_name.clone()
        } else {
            // Return empty string if course not found in config
            // The template will just use the course_id as fallback
            String::new()
        }
    }



    /// Generate filename for a template
    pub fn generate_filename(
        course_id: &str,
        template_type: &TemplateType,
        custom_title: Option<&str>,
    ) -> Result<String> {
        let date = Local::now().format("%Y-%m-%d").to_string();

        match template_type {
            TemplateType::Lecture => {
                Ok(format!("{}-{}-lecture.typ", date, course_id))
            }
            TemplateType::Assignment => {
                if let Some(title) = custom_title {
                    let sanitized_title = Validator::sanitize_filename(title);
                    Ok(format!("{}-{}-{}.typ", date, course_id, sanitized_title))
                } else {
                    Ok(format!("{}-{}-assignment.typ", date, course_id))
                }
            }
            TemplateType::Custom(name) => {
                let sanitized_name = Validator::sanitize_filename(name);
                Ok(format!("{}-{}-{}.typ", date, course_id, sanitized_name))
            }
        }
    }

    /// Validate template context
    pub fn validate_context(context: &TemplateContext) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        if context.author.is_empty() {
            warnings.push("Author name is empty".to_string());
        }

        if context.course_name.is_empty() {
            warnings.push(format!("Course name not found for {}", context.course_id));
        }

        if context.template_version.is_empty() {
            warnings.push("Template version is not specified".to_string());
        }

        if context.sections.is_empty() {
            warnings.push("No sections configured for template".to_string());
        }

        Ok(warnings)
    }
}

/// Template builder for more complex template creation
pub struct TemplateBuilder {
    context: TemplateContext,
    template_type: TemplateType,
}

impl TemplateBuilder {
    pub fn new(course_id: &str, config: &Config) -> Result<Self> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let course_name = TemplateEngine::resolve_course_name(course_id, config);
        let semester = StatusManager::get_current_semester(config);

        Ok(Self {
            context: TemplateContext {
                course_id: course_id.to_string(),
                course_name,
                title: String::new(),
                author: config.author.clone(),
                date,
                semester,
                template_version: config.template_version.clone(),
                sections: Vec::new(),
                custom_fields: HashMap::new(),
            },
            template_type: TemplateType::Lecture,
        })
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.context.title = title.to_string();
        self
    }

    pub fn with_type(mut self, template_type: TemplateType) -> Self {
        self.template_type = template_type;
        self
    }

    pub fn with_sections(mut self, sections: Vec<String>) -> Self {
        self.context.sections = sections;
        self
    }

    pub fn add_custom_field(mut self, key: &str, value: &str) -> Self {
        self.context.custom_fields.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&self) -> Result<String> {
        TemplateEngine::render_template(&self.context, &self.template_type)
    }

    pub fn build_with_filename(&self) -> Result<(String, String)> {
        let content = self.build()?;
        let filename = TemplateEngine::generate_filename(
            &self.context.course_id,
            &self.template_type,
            Some(&self.context.title),
        )?;

        Ok((content, filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_generate_lecture_filename() {
        let filename = TemplateEngine::generate_filename(
            "02101",
            &TemplateType::Lecture,
            None,
        ).unwrap();

        assert!(filename.contains("02101"));
        assert!(filename.contains("lecture"));
        assert!(filename.ends_with(".typ"));
    }

    #[test]
    fn test_generate_assignment_filename() {
        let filename = TemplateEngine::generate_filename(
            "02101",
            &TemplateType::Assignment,
            Some("Problem Set 1"),
        ).unwrap();

        assert!(filename.contains("02101"));
        assert!(filename.contains("problem-set-1"));
        assert!(filename.ends_with(".typ"));
    }

    #[test]
    fn test_sanitize_assignment_title() {
        let sanitized = Validator::sanitize_filename("Problem Set #1: Arrays & Pointers");
        assert_eq!(sanitized, "problem-set-1-arrays-pointers");
    }
}