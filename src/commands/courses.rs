//! Course management commands
//!
//! Thin command layer that delegates to core business logic.

use anyhow::Result;
use colored::Colorize;
use crate::config::get_config;
use crate::core::course_management::{CourseManager, get_common_courses};
use crate::core::validation::Validator;
use crate::ui::formatters::Formatters;
use crate::ui::output::{OutputManager, Status};

pub fn list_courses() -> Result<()> {
    let config = get_config()?;
    let courses = config.list_courses();

    let formatted_output = Formatters::format_course_list(&courses);
    println!("{}", formatted_output);

    if !courses.is_empty() {
        print_usage_examples();
    } else {
        println!("Add courses with: {}", "noter courses add 02101 \"Introduction to Programming\"".bright_white());
    }

    Ok(())
}

pub fn add_course(course_id: &str, course_name: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;

    let mut config = get_config()?;
    let mut manager = CourseManager::new(&mut config);

    match manager.add_course(course_id, course_name) {
        Ok(()) => {
            OutputManager::print_status(
                Status::Success,
                &format!("Added course: {} - {}",
                         course_id.yellow(),
                         course_name.green())
            );
            println!("You can now create notes with: {}",
                     format!("noter note {}", course_id).bright_white());
        }
        Err(e) => {
            OutputManager::print_status(Status::Warning, &e.to_string());
            println!("Use a different course ID or remove the existing one first.");
        }
    }

    Ok(())
}

pub fn remove_course(course_id: &str) -> Result<()> {
    Validator::validate_course_id(course_id)?;

    let mut config = get_config()?;
    let mut manager = CourseManager::new(&mut config);

    match manager.remove_course(course_id) {
        Ok(course_name) => {
            OutputManager::print_status(
                Status::Success,
                &format!("Removed course: {} - {}",
                         course_id.yellow(),
                         course_name.dimmed())
            );
        }
        Err(_) => {
            OutputManager::print_status(
                Status::Error,
                &format!("Course {} not found in your configuration.", course_id.yellow())
            );
            println!("Use {} to see available courses.", "noter courses list".bright_white());
        }
    }

    Ok(())
}

pub fn browse_common_courses() -> Result<()> {
    OutputManager::print_section("Common DTU Course Codes", Some("🎓"));

    let categories = get_common_courses();
    for (category, courses) in categories {
        println!("{}:", category.bright_cyan());
        for (course_id, course_name) in *courses {
            println!("  {} - {}", course_id.yellow(), course_name);
        }
        println!();
    }

    print_quick_add_examples();
    Ok(())
}

fn print_usage_examples() {

    OutputManager::print_command_examples(&[
        ("noter note 02101", "Create a lecture note"),
        ("noter assignment 02101 \"Problem Set 1\"", "Create assignment"),
        ("noter courses add 02103 \"Programming\"", "Add a new course"),
        ("noter recent 02101", "List recent notes"),
    ]);
}

fn print_quick_add_examples() {

    OutputManager::print_command_examples(&[
        ("noter courses add 02101 \"Introduction to Programming\"", ""),
        ("noter courses add 01005 \"Advanced Engineering Mathematics 1\"", ""),
        ("noter courses add 25200 \"Classical Physics 1\"", ""),
    ]);

    println!();
    println!("Use {} to see your configured courses.", "noter courses list".bright_white());
}