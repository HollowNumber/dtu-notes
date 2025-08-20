//! Core business logic modules
//!
//! This module contains the core domain logic separated from CLI commands
//! and presentation concerns.

pub mod course_management;
#[cfg(feature = "dev-tools")]
pub mod dev_data_generator;
pub mod directory_scanner;
pub mod file_operations;
pub mod github_template_fetcher;
#[cfg(feature = "dev-tools")]
pub mod sample_content;
pub mod search_engine;
pub mod setup_manager;
pub mod status_manager;
pub mod template;
pub mod typst_compiler;
pub mod validation;
