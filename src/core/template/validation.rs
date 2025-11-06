//! Template system validation
//!
//! Provides comprehensive validation for template configurations, engine capabilities,
//! file system state, and runtime contexts.

use crate::config::Config;
use crate::core::template::config::{
    EngineConfig, TemplateConfig, TemplateDefinition, TemplateVariant, ValidationRule,
    ValidationRuleType, VariableConfig,
};
use crate::core::template::context::TemplateContext;
use crate::core::template::discovery::{AvailableTemplate, TemplateDiscovery};
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

/// Validation severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Validation result with detailed information
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub category: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub location: Option<String>,
}

/// Template system validator
pub struct TemplateValidator;

impl TemplateValidator {
    /// Validate complete template system configuration
    pub fn validate_system(config: &Config) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Validate template configurations
        match TemplateDiscovery::load_template_configs(config) {
            Ok(template_configs) => {
                for (index, template_config) in template_configs.iter().enumerate() {
                    let mut config_issues = Self::validate_template_config(template_config)?;

                    // Add location context to issues
                    for issue in &mut config_issues {
                        issue.location = Some(format!("template_config[{}]", index));
                    }

                    issues.extend(config_issues);
                }

                // Cross-configuration validation
                issues.extend(Self::validate_cross_configurations(&template_configs)?);
            }
            Err(e) => {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    category: "configuration".to_string(),
                    message: format!("Failed to load template configurations: {}", e),
                    suggestion: Some(
                        "Check template directory and configuration files".to_string(),
                    ),
                    location: None,
                });
            }
        }

        // Validate file system state
        issues.extend(Self::validate_file_system(config)?);

        // Validate engine compatibility
        issues.extend(Self::validate_engine_compatibility(config)?);

        Ok(issues)
    }

    /// Validate individual template configuration
    pub fn validate_template_config(config: &TemplateConfig) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Validate metadata
        issues.extend(Self::validate_metadata(&config.metadata));

        // Validate template definitions
        for (index, template) in config.templates.iter().enumerate() {
            let mut template_issues = Self::validate_template_definition(template)?;
            for issue in &mut template_issues {
                issue.location = Some(format!("templates[{}]", index));
            }
            issues.extend(template_issues);
        }

        // Validate variants
        if let Some(variants) = &config.variants {
            for (index, variant) in variants.iter().enumerate() {
                let mut variant_issues =
                    Self::validate_template_variant(variant, &config.templates)?;
                for issue in &mut variant_issues {
                    issue.location = Some(format!("variants[{}]", index));
                }
                issues.extend(variant_issues);
            }
        }

        // Validate course mapping
        if let Some(mapping) = &config.course_mapping {
            issues.extend(Self::validate_course_mapping(mapping));
        }

        // Validate engine configuration
        if let Some(engine) = &config.engine {
            issues.extend(Self::validate_engine_config(engine)?);
        }

        Ok(issues)
    }

    /// Validate template context before rendering
    pub fn validate_template_context(
        context: &TemplateContext,
        template_def: &TemplateDefinition,
        variant: Option<&TemplateVariant>,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Basic context validation
        if context.author.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "context".to_string(),
                message: "Author name is empty".to_string(),
                suggestion: Some("Set author name in configuration".to_string()),
                location: Some("context.author".to_string()),
            });
        }

        if context.course_id.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "context".to_string(),
                message: "Course ID is required".to_string(),
                suggestion: None,
                location: Some("context.course_id".to_string()),
            });
        }

        if context.course_name.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "context".to_string(),
                message: format!("Course name not found for {}", context.course_id),
                suggestion: Some("Add course to configuration".to_string()),
                location: Some("context.course_name".to_string()),
            });
        }

        // Engine-specific validation
        if context.engine_config.validation.validate_variables {
            issues.extend(Self::validate_context_variables(
                context,
                &context.engine_config.variables,
            ));
        }

        // Template-specific validation
        issues.extend(Self::validate_template_sections(
            template_def,
            variant,
            context,
        ));

        // Custom validation rules
        for rule in &context.engine_config.validation.custom_rules {
            issues.extend(Self::apply_custom_validation_rule(
                rule,
                context,
                template_def,
            )?);
        }

        Ok(issues)
    }

    #[allow(dead_code)]
    /// Validate available template accessibility
    pub fn validate_available_template(template: &AvailableTemplate) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        if !template.is_accessible {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "accessibility".to_string(),
                message: format!("Template file not accessible: {}", template.file_path),
                suggestion: Some("Check file permissions and path".to_string()),
                location: Some(template.file_path.clone()),
            });
        }

        // Validate file existence
        if !Path::new(&template.file_path).exists() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "file_system".to_string(),
                message: format!("Template file does not exist: {}", template.file_path),
                suggestion: Some("Run 'noter template update' to download templates".to_string()),
                location: Some(template.file_path.clone()),
            });
        }

        // Validate variants consistency
        for _ in &template.variants {
            if !template.definition.supports_variants {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: "configuration".to_string(),
                    message: format!(
                        "Template '{}' has variants but doesn't support them",
                        template.definition.name
                    ),
                    suggestion: Some("Set supports_variants to true".to_string()),
                    location: Some(format!("template.{}", template.definition.name)),
                });
            }
        }

        issues
    }

    // Private validation methods

    fn validate_metadata(
        metadata: &crate::core::template::config::TemplateMetadata,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        if metadata.name.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "metadata".to_string(),
                message: "Template package name is required".to_string(),
                suggestion: None,
                location: Some("metadata.name".to_string()),
            });
        }

        if metadata.version.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "metadata".to_string(),
                message: "Template package version is required".to_string(),
                suggestion: None,
                location: Some("metadata.version".to_string()),
            });
        }

        // Validate semantic version format
        if semver::Version::parse(&metadata.version).is_err() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "metadata".to_string(),
                message: format!(
                    "Version '{}' is not a valid semantic version",
                    metadata.version
                ),
                suggestion: Some("Use format like '1.0.0'".to_string()),
                location: Some("metadata.version".to_string()),
            });
        }

        issues
    }

    fn validate_template_definition(template: &TemplateDefinition) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        if template.name.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "template".to_string(),
                message: "Template name is required".to_string(),
                suggestion: None,
                location: Some("name".to_string()),
            });
        }

        if template.file.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "template".to_string(),
                message: "Template file path is required".to_string(),
                suggestion: None,
                location: Some("file".to_string()),
            });
        }

        if template.function.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "template".to_string(),
                message: "Template function name is required".to_string(),
                suggestion: None,
                location: Some("function".to_string()),
            });
        }

        // Validate file extension
        if !template.file.ends_with(".typ") {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "template".to_string(),
                message: format!(
                    "Template file '{}' should have .typ extension",
                    template.file
                ),
                suggestion: Some("Use .typ extension for Typst templates".to_string()),
                location: Some("file".to_string()),
            });
        }

        // Validate course types format
        if let Some(course_types) = &template.course_types {
            for course_type in course_types {
                if course_type.trim().is_empty() {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Warning,
                        category: "template".to_string(),
                        message: "Empty course type found".to_string(),
                        suggestion: Some("Remove empty course types".to_string()),
                        location: Some("course_types".to_string()),
                    });
                }
            }
        }

        Ok(issues)
    }

    fn validate_template_variant(
        variant: &TemplateVariant,
        templates: &[TemplateDefinition],
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Check if base template exists
        let base_template_exists = templates.iter().any(|t| t.name == variant.template);
        if !base_template_exists {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "variant".to_string(),
                message: format!("Base template '{}' not found", variant.template),
                suggestion: Some("Define the base template first".to_string()),
                location: Some("template".to_string()),
            });
        }

        if variant.name.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "variant".to_string(),
                message: "Variant name is required".to_string(),
                suggestion: None,
                location: Some("name".to_string()),
            });
        }

        if variant.course_types.is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "variant".to_string(),
                message: "Variant should specify course types".to_string(),
                suggestion: Some("Add course_types to improve template selection".to_string()),
                location: Some("course_types".to_string()),
            });
        }

        Ok(issues)
    }

    fn validate_course_mapping(mapping: &HashMap<String, String>) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for (pattern, course_type) in mapping {
            if pattern.trim().is_empty() {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: "course_mapping".to_string(),
                    message: "Empty course pattern found".to_string(),
                    suggestion: Some("Remove empty patterns".to_string()),
                    location: None,
                });
            }

            if course_type.trim().is_empty() {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: "course_mapping".to_string(),
                    message: format!("Empty course type for pattern '{}'", pattern),
                    suggestion: Some("Provide a valid course type".to_string()),
                    location: None,
                });
            }

            // Validate pattern format (basic regex check)
            if pattern.contains("xxx") && !pattern.ends_with("xxx") {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Info,
                    category: "course_mapping".to_string(),
                    message: format!("Pattern '{}' might be malformed", pattern),
                    suggestion: Some("Use patterns like '01xxx' for course prefixes".to_string()),
                    location: None,
                });
            }
        }

        issues
    }

    fn validate_engine_config(engine: &EngineConfig) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Validate version compatibility
        if semver::Version::parse(&engine.compatibility.minimum_noter_version).is_err() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "engine".to_string(),
                message: format!(
                    "Invalid minimum_noter_version: {}",
                    engine.compatibility.minimum_noter_version
                ),
                suggestion: Some("Use semantic versioning format".to_string()),
                location: Some("compatibility.minimum_noter_version".to_string()),
            });
        }

        // Validate rendering limits
        if engine.rendering.timeout_seconds == 0 {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "engine".to_string(),
                message: "Rendering timeout is set to 0 seconds".to_string(),
                suggestion: Some("Set a reasonable timeout (e.g., 30 seconds)".to_string()),
                location: Some("rendering.timeout_seconds".to_string()),
            });
        }

        if engine.rendering.max_concurrent == 0 {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "engine".to_string(),
                message: "Max concurrent processing is set to 0".to_string(),
                suggestion: Some("Set at least 1 for processing capability".to_string()),
                location: Some("rendering.max_concurrent".to_string()),
            });
        }

        Ok(issues)
    }

    fn validate_cross_configurations(configs: &[TemplateConfig]) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        let mut template_names = HashMap::new();

        // Check for duplicate template names across configurations
        for (config_index, config) in configs.iter().enumerate() {
            for template in &config.templates {
                if let Some(existing_config_index) = template_names.get(&template.name) {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Warning,
                        category: "cross_config".to_string(),
                        message: format!(
                            "Template '{}' is defined in multiple configurations (config {} and {})",
                            template.name, existing_config_index, config_index
                        ),
                        suggestion: Some("Consider renaming templates to avoid conflicts".to_string()),
                        location: Some(format!("template.{}", template.name)),
                    });
                } else {
                    template_names.insert(template.name.clone(), config_index);
                }
            }
        }

        Ok(issues)
    }

    fn validate_file_system(config: &Config) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Check template directory
        if !Path::new(&config.paths.templates_dir).exists() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "file_system".to_string(),
                message: format!(
                    "Template directory does not exist: {}",
                    config.paths.templates_dir
                ),
                suggestion: Some("Run setup or create the directory manually".to_string()),
                location: Some("paths.templates_dir".to_string()),
            });
        }

        // Check typst packages directory
        if !Path::new(&config.paths.typst_packages_dir).exists() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Info,
                category: "file_system".to_string(),
                message: format!(
                    "Typst packages directory does not exist: {}",
                    config.paths.typst_packages_dir
                ),
                suggestion: Some("Templates will be downloaded when needed".to_string()),
                location: Some("paths.typst_packages_dir".to_string()),
            });
        }

        Ok(issues)
    }

    fn validate_engine_compatibility(config: &Config) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Check if current version meets minimum requirements
        let current_version = env!("CARGO_PKG_VERSION");

        if let Ok(configs) = TemplateDiscovery::load_template_configs(config) {
            for template_config in &configs {
                if let Some(engine) = &template_config.engine {
                    if let (Ok(current), Ok(required)) = (
                        semver::Version::parse(current_version),
                        semver::Version::parse(&engine.compatibility.minimum_noter_version),
                    ) {
                        if current < required {
                            issues.push(ValidationIssue {
                                severity: ValidationSeverity::Error,
                                category: "compatibility".to_string(),
                                message: format!(
                                    "Template '{}' requires noter version {} but current is {}",
                                    template_config.metadata.name,
                                    engine.compatibility.minimum_noter_version,
                                    current_version
                                ),
                                suggestion: Some("Update noter to the latest version".to_string()),
                                location: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    fn validate_context_variables(
        context: &TemplateContext,
        variable_config: &VariableConfig,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for required_var in &variable_config.builtin_variables {
            if !context.variables.contains_key(required_var) {
                issues.push(ValidationIssue {
                    severity: if variable_config.allow_undefined {
                        ValidationSeverity::Warning
                    } else {
                        ValidationSeverity::Error
                    },
                    category: "variables".to_string(),
                    message: format!("Required variable '{}' is missing", required_var),
                    suggestion: Some("Provide all required variables in context".to_string()),
                    location: Some(format!("variables.{}", required_var)),
                });
            }
        }

        issues
    }

    fn validate_template_sections(
        template_def: &TemplateDefinition,
        variant: Option<&TemplateVariant>,
        context: &TemplateContext,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check if custom sections are provided when template doesn't have defaults
        if template_def.default_sections.is_empty() && context.sections.is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Info,
                category: "sections".to_string(),
                message: format!("Template '{}' has no default sections", template_def.name),
                suggestion: Some("Consider providing custom sections".to_string()),
                location: Some("context.sections".to_string()),
            });
        }

        // Validate variant-specific sections
        if let Some(variant) = variant {
            if let Some(override_sections) = &variant.override_sections {
                if override_sections.is_empty() {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Warning,
                        category: "sections".to_string(),
                        message: format!("Variant '{}' has empty override_sections", variant.name),
                        suggestion: Some(
                            "Remove override_sections or provide sections".to_string(),
                        ),
                        location: Some("variant.override_sections".to_string()),
                    });
                }
            }
        }

        issues
    }

    fn apply_custom_validation_rule(
        rule: &ValidationRule,
        context: &TemplateContext,
        _template_def: &TemplateDefinition,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        match &rule.rule_type {
            ValidationRuleType::RequiredVariables => {
                if let Some(required_vars) = rule.parameters.get("variables") {
                    let vars: Vec<&str> = required_vars.split(',').collect();
                    for var in vars {
                        if !context.variables.contains_key(var.trim()) {
                            issues.push(ValidationIssue {
                                severity: ValidationSeverity::Error,
                                category: "custom_rule".to_string(),
                                message: rule.error_message.clone(),
                                suggestion: Some(format!("Provide variable '{}'", var.trim())),
                                location: Some(format!("variables.{}", var.trim())),
                            });
                        }
                    }
                }
            }
            ValidationRuleType::VariablePattern => {
                if let (Some(variable), Some(pattern)) = (
                    rule.parameters.get("variable"),
                    rule.parameters.get("pattern"),
                ) {
                    if let Some(value) = context.variables.get(variable) {
                        if let Ok(regex) = Regex::new(pattern) {
                            if !regex.is_match(value) {
                                issues.push(ValidationIssue {
                                    severity: ValidationSeverity::Warning,
                                    category: "custom_rule".to_string(),
                                    message: rule.error_message.clone(),
                                    suggestion: Some(format!(
                                        "Variable '{}' should match pattern '{}'",
                                        variable, pattern
                                    )),
                                    location: Some(format!("variables.{}", variable)),
                                });
                            }
                        }
                    }
                }
            }
            ValidationRuleType::MaxFileSize => {
                // File size validation would be implemented here
                // Currently not applicable for template context validation
            }
            ValidationRuleType::Custom(_) => {
                // Custom validation logic would be implemented here
                // This could involve calling external validators or scripts
            }
        }

        Ok(issues)
    }

    /// Format validation issues for display
    pub fn format_validation_report(issues: &[ValidationIssue]) -> String {
        if issues.is_empty() {
            return "No validation issues found".to_string();
        }

        let mut report = String::new();
        let errors = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .count();
        let warnings = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Warning)
            .count();
        let infos = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Info)
            .count();

        report.push_str(&format!(
            "Validation Report: {} errors, {} warnings, {} info\n\n",
            errors, warnings, infos
        ));

        for issue in issues {
            let icon = match issue.severity {
                ValidationSeverity::Error => "❌",
                ValidationSeverity::Warning => "⚠️",
                ValidationSeverity::Info => "ℹ️",
            };

            report.push_str(&format!(
                "{} [{}] {}\n",
                icon, issue.category, issue.message
            ));

            if let Some(location) = &issue.location {
                report.push_str(&format!("   Location: {}\n", location));
            }

            if let Some(suggestion) = &issue.suggestion {
                report.push_str(&format!("   Suggestion: {}\n", suggestion));
            }

            report.push('\n');
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_issue_creation() {
        let issue = ValidationIssue {
            severity: ValidationSeverity::Error,
            category: "test".to_string(),
            message: "Test message".to_string(),
            suggestion: Some("Test suggestion".to_string()),
            location: Some("test.location".to_string()),
        };

        assert_eq!(issue.severity, ValidationSeverity::Error);
        assert_eq!(issue.category, "test");
    }

    #[test]
    fn test_format_validation_report() {
        let issues = vec![
            ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "test".to_string(),
                message: "Test error".to_string(),
                suggestion: None,
                location: None,
            },
            ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "test".to_string(),
                message: "Test warning".to_string(),
                suggestion: Some("Fix this".to_string()),
                location: Some("test.field".to_string()),
            },
        ];

        let report = TemplateValidator::format_validation_report(&issues);
        assert!(report.contains("1 errors, 1 warnings, 0 info"));
        assert!(report.contains("❌"));
        assert!(report.contains("⚠️"));
    }
}
