//! Development tools for simulating high-yield note environments
//!
//! This module provides CLI commands for generating realistic test data including
//! notes, assignments, and course structures for development and testing.

use anyhow::Result;

use crate::config::get_config;
use crate::core::dev_data_generator::DevDataGenerator;

/// Generate a high-yield simulation with many notes and assignments
pub fn simulate_high_yield_setup() -> Result<()> {
    let config = get_config()?;
    let mut generator = DevDataGenerator::new();
    generator.generate_high_yield_simulation(&config)?;
    Ok(())
}

/// Generate sample data with specific parameters
pub fn generate_sample_data(
    courses: usize,
    notes_per_course: usize,
    assignments_per_course: usize,
) -> Result<()> {
    let config = get_config()?;
    let mut generator = DevDataGenerator::new();
    generator.generate_sample_data(&config, courses, notes_per_course, assignments_per_course)?;
    Ok(())
}

/// Clean all generated dev data
pub fn clean_dev_data() -> Result<()> {
    let config = get_config()?;
    DevDataGenerator::clean_dev_data(&config)?;
    Ok(())
}
