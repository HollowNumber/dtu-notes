use anyhow::Result;

pub mod notes;
pub mod assignments;
pub mod typst;
pub mod search;
pub mod setup;
pub mod info;
pub mod config_cmd;
pub mod courses;

use crate::{Commands, ConfigAction, CourseAction};

pub fn execute_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Note { course_id } => notes::create_note(course_id),
        Commands::Assignment { course_id, title } => assignments::create_assignment(course_id, title),
        Commands::Compile { filepath } => typst::compile_file(filepath),
        Commands::Watch { filepath } => typst::watch_file(filepath),
        Commands::Recent { course_id } => notes::list_recent(course_id),
        Commands::Setup => setup::setup_repository(),
        Commands::Index { course_id } => notes::create_index(course_id),
        Commands::Search { query } => search::search_notes(query),
        Commands::Courses { action } => execute_course_action(action),
        Commands::Clean => typst::clean_files(),
        Commands::Status => info::show_enhanced_status(),
        Commands::Open {course_id} => notes::open_recent(course_id),
        Commands::Semester => info::show_semester(),
        Commands::Config { action } => execute_config_action(action),
    }
}

fn execute_config_action(action: &ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Show => config_cmd::show_config(),
        ConfigAction::SetAuthor { name } => config_cmd::set_author(name),
        ConfigAction::SetEditor { editor } => config_cmd::set_editor(editor),
        ConfigAction::Reset => config_cmd::reset_config(),
        ConfigAction::Path => config_cmd::show_config_path(),
        ConfigAction::Check => config_cmd::check_config(),
    }
}

fn execute_course_action(action: &CourseAction) -> Result<()> {
    match action {
        CourseAction::List => courses::list_courses(),
        CourseAction::Add { course_id, course_name } => courses::add_course(course_id, course_name),
        CourseAction::Remove { course_id } => courses::remove_course(course_id),
        CourseAction::Browse => courses::browse_common_courses(),
    }
}
