use anyhow::Result;
use colored::*;

use crate::config::get_config;
use crate::utils::validate_course_id;

pub fn list_courses() -> Result<()> {
    let config = get_config()?;
    let courses = config.list_courses();

    if courses.is_empty() {
        println!("{} No courses configured.", "ℹ️".blue());
        println!("Add courses with: {}", "noter courses add 02101 \"Introduction to Programming\"".bright_white());
        return Ok(());
    }

    println!("{} Your DTU Courses:", "🎓".blue());
    println!();

    for (course_id, course_name) in courses {
        println!("  {} - {}", course_id.yellow(), course_name);
    }

    println!();
    println!("{} Total: {} courses", "📊".blue(), config.courses.len().to_string().green());
    println!();
    println!("{}", "Usage Examples:".green());
    println!("  noter note 02101                           # Create a lecture note");
    println!("  noter assignment 02101 \"Problem Set 1\"     # Create assignment");
    println!("  noter courses add 02103 \"Programming\"       # Add a new course");
    println!("  noter recent 02101                         # List recent notes");

    Ok(())
}

pub fn add_course(course_id: &str, course_name: &str) -> Result<()> {
    validate_course_id(course_id)?;

    let mut config = get_config()?;

    if config.courses.contains_key(course_id) {
        println!("{} Course {} already exists: {}", "⚠️".yellow(), course_id.yellow(),
                 config.courses.get(course_id).unwrap().dimmed());
        println!("Use a different course ID or remove the existing one first.");
        return Ok(());
    }

    config.add_course(course_id.to_string(), course_name.to_string())?;

    println!("{} Added course: {} - {}", "✅".green(), course_id.yellow(), course_name.green());
    println!("You can now create notes with: {}", format!("noter note {}", course_id).bright_white());

    Ok(())
}

pub fn remove_course(course_id: &str) -> Result<()> {
    validate_course_id(course_id)?;

    let mut config = get_config()?;

    if let Some(course_name) = config.courses.get(course_id) {
        let course_name = course_name.clone(); // Clone to avoid borrow issues
        config.remove_course(course_id)?;
        println!("{} Removed course: {} - {}", "✅".green(), course_id.yellow(), course_name.dimmed());
    } else {
        println!("{} Course {} not found in your configuration.", "❌".red(), course_id.yellow());
        println!("Use {} to see available courses.", "noter courses list".bright_white());
    }

    Ok(())
}

pub fn browse_common_courses() -> Result<()> {
    println!("{} Common DTU Course Codes:", "🎓".blue());
    println!();

    // Organize courses by category
    let categories = [
        ("Mathematics & Computer Science", vec![
            ("01005", "Advanced Engineering Mathematics 1"),
            ("01006", "Advanced Engineering Mathematics 2"),
            ("01017", "Discrete Mathematics"),
            ("01035", "Mathematics 1"),
            ("01037", "Mathematics 2"),
            ("02101", "Introduction to Programming"),
            ("02102", "Algorithms and Data Structures"),
            ("02105", "Algorithms and Data Structures 2"),
            ("02110", "Algorithms and Data Structures"),
            ("02157", "Functional Programming"),
            ("02158", "Concurrent Programming"),
            ("02159", "Operating Systems"),
            ("02180", "Introduction to Artificial Intelligence"),
            ("02201", "Introduction to Database Systems"),
            ("02393", "Programming in C++"),
            ("02450", "Introduction to Machine Learning and Data Mining"),
        ]),
        ("Physics & Engineering", vec![
            ("10020", "Advanced Engineering Mathematics"),
            ("10333", "Solid Mechanics 1"),
            ("22100", "Electronics 1"),
            ("22101", "Electronics 2"),
            ("25100", "Introduction to Physics and Nanotechnology"),
            ("25200", "Classical Physics 1"),
            ("25201", "Classical Physics 2"),
            ("28000", "Introduction to Environmental Engineering"),
            ("31001", "Fluid Mechanics 1"),
            ("31002", "Fluid Mechanics 2"),
        ]),
        ("Chemistry & Materials Science", vec![
            ("28230", "Introduction to Environmental Chemistry"),
            ("28240", "Chemical and Biochemical Engineering Fundamentals"),
            ("28350", "Surface Chemistry and Catalysis"),
        ]),
        ("Civil & Mechanical Engineering", vec![
            ("11034", "Introduction to Building Design"),
            ("11035", "Structural Analysis"),
            ("42101", "Thermal Energy Systems"),
            ("42435", "Advanced Thermodynamics"),
        ]),
        ("Biotechnology & Chemical Engineering", vec![
            ("27002", "Life Science - Chemistry, Biochemistry and Cells"),
            ("27003", "Life Science - Molecular Biotechnology"),
            ("28330", "Introduction to Chemical and Biochemical Engineering"),
        ]),
        ("Environmental Engineering", vec![
            ("12100", "Quantitative Sustainability Analysis"),
            ("12132", "Climate Change - Mitigation Technologies"),
        ]),
    ];

    for (category, courses) in categories {
        println!("{}:", category.bright_cyan());
        for (course_id, course_name) in courses {
            println!("  {} - {}", course_id.yellow(), course_name);
        }
        println!();
    }

    println!("{}", "Quick Add Examples:".green());
    println!("  noter courses add 02101 \"Introduction to Programming\"");
    println!("  noter courses add 01005 \"Advanced Engineering Mathematics 1\"");
    println!("  noter courses add 25200 \"Classical Physics 1\"");
    println!();
    println!("Use {} to see your configured courses.", "noter courses list".bright_white());

    Ok(())
}