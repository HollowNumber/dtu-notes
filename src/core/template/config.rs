//! Template configuration types and parsing
//!
//! Contains all configuration structures for template packages,
//! including engine capabilities and template definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig {
    pub metadata: TemplateMetadata,
    pub templates: Vec<TemplateDefinition>,
    pub variants: Option<Vec<TemplateVariant>>,
    pub course_mapping: Option<HashMap<String, String>>,
    pub engine: Option<EngineConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateDefinition {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub file: String,
    pub function: String,
    pub supports_variants: bool,
    pub course_types: Option<Vec<String>>,
    pub default_sections: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateVariant {
    pub template: String,
    pub name: String,
    pub display_name: String,
    pub course_types: Vec<String>,
    pub file: String,
    pub function: Option<String>,
    pub additional_sections: Option<Vec<String>>,
    pub override_sections: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineConfig {
    pub features: EngineFeatures,
    pub compatibility: CompatibilityConfig,
    pub processing: ProcessingConfig,
    pub variables: VariableConfig,
    pub validation: ValidationConfig,
    pub rendering: RenderingConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineFeatures {
    pub supports_conditionals: bool,
    pub supports_custom_sections: bool,
    pub supports_dynamic_content: bool,
    pub supports_expressions: bool,
    pub supports_includes: bool,
    pub supports_loops: bool,
    pub supported_formats: Vec<String>,
    pub supports_metadata: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompatibilityConfig {
    pub minimum_noter_version: String,
    pub required_typst_version: Option<String>,
    pub supported_platforms: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingConfig {
    pub encoding: String,
    pub line_endings: LineEndingStyle,
    pub preserve_formatting: bool,
    pub minify_output: bool,
    pub hooks: ProcessingHooks,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableConfig {
    pub substitution_syntax: String,
    pub case_sensitive: bool,
    pub allow_undefined: bool,
    pub undefined_default: Option<String>,
    pub builtin_variables: Vec<String>,
    pub transformations: Vec<VariableTransformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationConfig {
    pub validate_syntax: bool,
    pub validate_variables: bool,
    pub validate_references: bool,
    pub custom_rules: Vec<ValidationRule>,
    pub strict_validation: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderingConfig {
    pub timeout_seconds: u64,
    pub max_memory_mb: Option<u32>,
    pub enable_caching: bool,
    pub cache_duration_minutes: u32,
    pub parallel_processing: bool,
    pub max_concurrent: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LineEndingStyle {
    Unix,
    Windows,
    Mac,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingHooks {
    pub pre_process: Vec<String>,
    pub post_process: Vec<String>,
    pub on_error: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableTransformation {
    pub name: String,
    pub transformation_type: TransformationType,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransformationType {
    Uppercase,
    Lowercase,
    TitleCase,
    DateFormat,
    RegexReplace,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationRule {
    pub name: String,
    pub rule_type: ValidationRuleType,
    pub parameters: HashMap<String, String>,
    pub error_message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValidationRuleType {
    RequiredVariables,
    VariablePattern,
    MaxFileSize,
    Custom(String),
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            features: EngineFeatures {
                supports_conditionals: false,
                supports_custom_sections: true,
                supports_dynamic_content: false,
                supports_expressions: false,
                supports_includes: false,
                supports_loops: false,
                supported_formats: vec!["typst".to_string()],
                supports_metadata: true,
            },
            compatibility: CompatibilityConfig {
                minimum_noter_version: "0.4.0".to_string(),
                required_typst_version: None,
                supported_platforms: vec![
                    "windows".to_string(),
                    "macos".to_string(),
                    "linux".to_string(),
                ],
                dependencies: vec![],
            },
            processing: ProcessingConfig {
                encoding: "utf-8".to_string(),
                line_endings: LineEndingStyle::Auto,
                preserve_formatting: true,
                minify_output: false,
                hooks: ProcessingHooks {
                    pre_process: vec![],
                    post_process: vec![],
                    on_error: vec![],
                },
            },
            variables: VariableConfig {
                substitution_syntax: "{{var}}".to_string(),
                case_sensitive: false,
                allow_undefined: false,
                undefined_default: None,
                builtin_variables: vec![
                    "author".to_string(),
                    "date".to_string(),
                    "course_id".to_string(),
                    "title".to_string(),
                ],
                transformations: vec![],
            },
            validation: ValidationConfig {
                validate_syntax: false,
                validate_variables: false,
                validate_references: false,
                custom_rules: vec![],
                strict_validation: false,
            },
            rendering: RenderingConfig {
                timeout_seconds: 30,
                max_memory_mb: None,
                enable_caching: true,
                cache_duration_minutes: 60,
                parallel_processing: false,
                max_concurrent: 1,
            },
        }
    }
}
