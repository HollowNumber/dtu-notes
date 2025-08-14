use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::get_config;
use crate::utils::get_course_name;

pub fn setup_repository() -> Result<()> {
    let config = get_config()?;

    println!("{} Setting up DTU notes repository...", "🏗️".blue());
    println!(
        "Using configuration from: {}",
        crate::config::Config::config_file_path()?
            .display()
            .to_string()
            .dimmed()
    );
    println!();

    // Create directory structure based on config (excluding templates - we'll copy those)
    let dirs = [
        &config.paths.notes_dir,
        &format!("{}/courses", config.paths.obsidian_dir),
        &format!("{}/weekly-reviews", config.paths.obsidian_dir),
        &format!("{}/concept-maps", config.paths.obsidian_dir),
    ];

    println!("{} Creating directory structure...", "📁".blue());
    for dir in &dirs {
        fs::create_dir_all(dir)?;
        println!("  Created: {}", dir.dimmed());
    }

    // Copy templates from repository to local directories and install to Typst
    install_templates_to_typst()?;


    // Create sample DTU courses based on popular ones
    let sample_courses = [
        "02101", // Introduction to Programming
        "02102", // Algorithms and Data Structures
        "01005", // Advanced Engineering Mathematics 1
        "01006", // Advanced Engineering Mathematics 2
        "25200", // Classical Physics 1
        "22100", // Electronics 1
    ];

    println!();
    println!("{} Creating sample course directories...", "📚".blue());
    for course in &sample_courses {
        let course_dir = format!("{}/{}", config.paths.notes_dir, course);
        fs::create_dir_all(format!("{}/lectures", course_dir))?;
        fs::create_dir_all(format!("{}/assignments", course_dir))?;

        let course_name = get_course_name(course);
        if !course_name.is_empty() {
            println!("  {} - {}", course.yellow(), course_name);
        } else {
            println!("  {}", course.yellow());
        }
    }

    // Create a sample README
    create_readme(&config)?;

    // Create sample .gitignore
    create_gitignore(&config)?;

    // Summary
    println!();
    println!(
        "{} Setup completed successfully! {}",
        "✅".green(),
        "🎉".yellow()
    );
    println!();

    println!("{} Sample DTU courses created:", "📚".blue());
    for course in &sample_courses {
        let course_name = get_course_name(course);
        println!("   {} - {}", course, course_name);
    }

    println!();
    println!("{} Next steps:", "📝".green());
    println!("1. Update your author name:");
    println!(
        "   {}",
        "noter config set-author \"Your Full Name\"".bright_white()
    );
    println!();
    println!("2. Start taking notes:");
    println!("   {}", "noter note 02101".bright_white());
    println!();
    println!("3. Check your setup:");
    println!("   {}", "noter config show".bright_white());

    Ok(())
}

fn install_templates_to_typst() -> Result<()> {
    println!("{} Installing DTU templates...", "📦".blue());

    // Check if template directory exists in current working directory
    let repo_template_dir = Path::new("templates");
    if !repo_template_dir.exists() {
        println!("  {} No template directory found in current directory", "⚠️".yellow());
        println!("  {} Make sure you're running this from the repository root", "💡".blue());
        return Ok(());
    }

    let config = get_config()?;
    let local_template_dir = Path::new(&config.paths.templates_dir);

    // Skip copying to local templates if source and destination are the same
    if repo_template_dir.canonicalize()? != local_template_dir.canonicalize().unwrap_or_else(|_| local_template_dir.to_path_buf()) {
        println!("  {} Copying templates to local directory...", "📋".blue());
        println!("    From: {}", repo_template_dir.display().to_string().dimmed());
        println!("    To: {}", local_template_dir.display().to_string().dimmed());

        fs::create_dir_all(&local_template_dir)?;
        copy_dir_recursive(&repo_template_dir, &local_template_dir)?;
        println!("  {} Templates copied to local directory!", "✅".green());
    } else {
        println!("  {} Templates already in local directory (same as source)", "ℹ️".blue());
    }

    // Install templates to Typst local packages
    let typst_local_dir = get_typst_local_packages_dir()?;
    println!("  {} Installing to Typst packages...", "📦".blue());
    println!("    To: {}", typst_local_dir.display().to_string().dimmed());

    fs::create_dir_all(&typst_local_dir)?;
    copy_template_contents(&repo_template_dir, &typst_local_dir)?;

    println!("  {} DTU templates installed successfully!", "✅".green());

    // List what was installed
    if let Ok(entries) = fs::read_dir(&repo_template_dir) {
        println!("  {} Available templates:", "📋".blue());
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        println!("    • {}", name.green());
                    }
                }
            }
        }
    }

    println!();
    println!("  {} Templates are now available in:", "ℹ️".blue());
    println!("    • Local: {} (for editing)", local_template_dir.display().to_string().dimmed());
    println!("    • Typst: {} (for compilation)", typst_local_dir.display().to_string().dimmed());

    Ok(())
}

// Copy template contents (not the template directory itself)
fn copy_template_contents(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        return Err(anyhow::anyhow!("Source is not a directory: {}", src.display()));
    }

    fs::create_dir_all(dst)?;

    // Copy each item from templates/ to the destination
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}


fn get_typst_local_packages_dir() -> Result<std::path::PathBuf> {
    let typst_dir = if cfg!(target_os = "windows") {
        // Windows: %LocalAppData%\typst\packages\local
        let local_app_data = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .map_err(|_| anyhow::anyhow!("Could not find LocalAppData or AppData environment variable"))?;
        std::path::PathBuf::from(local_app_data)
            .join("typst")
            .join("packages")
            .join("local")
    } else if cfg!(target_os = "macos") {
        // macOS: ~/Library/Application Support/typst/packages/local
        dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?
            .join("typst")
            .join("packages")
            .join("local")
    } else {
        // Linux: ~/.local/share/typst/packages/local
        dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?
            .join("typst")
            .join("packages")
            .join("local")
    };

    Ok(typst_dir)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        return Err(anyhow::anyhow!("Source is not a directory: {}", src.display()));
    }

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // Handle file copying with better error handling for Windows
            match fs::copy(&src_path, &dst_path) {
                Ok(_) => {},
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        // Try to remove the destination file first if it exists
                        if dst_path.exists() {
                            match fs::remove_file(&dst_path) {
                                Ok(_) => {
                                    // Now try copying again
                                    fs::copy(&src_path, &dst_path)?;
                                },
                                Err(_) => {
                                    println!("  {} Warning: Could not overwrite {}", "⚠️".yellow(), dst_path.display());
                                    println!("    File may be in use. Close any editors and try again.");
                                    continue;
                                }
                            }
                        } else {
                            return Err(e.into());
                        }
                    } else {
                        return Err(e.into());
                    }
                }
            }
        }
    }

    Ok(())
}



fn create_readme(config: &crate::config::Config) -> Result<()> {
    let readme_content = format!(
        r#"# DTU Notes Repository

This repository contains lecture notes and assignments for DTU courses, organized using the DTU Notes CLI tool.

## Structure
```

{}/                    # Course notes
├── 02101/
│   ├── lectures/      # Lecture notes (.typ files)
│   └── assignments/   # Assignment files (.typ files)
└── [other courses]/

{}/           # Obsidian vault (optional)
├── courses/           # Course index files
├── weekly-reviews/    # Weekly review notes
└── concept-maps/      # Concept mapping notes

{}/              # Typst templates
└── dtu-template/      # DTU official templates
```
## Getting Started

1. **Create a lecture note:**
   ```bash
   noter note 02101
   ```

2. **Create an assignment:**
   ```bash
   noter assignment 02101 "Problem Set 1"
   ```

3. **Compile to PDF:**
   ```bash
   noter compile path/to/file.typ
   ```

4. **Search through notes:**
   ```bash
   noter search "algorithms"
   ```

## Configuration

Your configuration is stored at: `{}`

- Author: {}
- Template Version: {}

Update configuration:
```bash
noter config set-author "Your Name"
noter config set-editor code
noter config show
```
```


## Template System

This setup uses the DTU official templates following the DTU Design Guide 2018.
Templates are located in `{}/dtu-template/`.

## Tips

- Use `noter recent 02101` to see recent notes for a course
- Use `noter courses` to see all available DTU course codes  
- Use `noter clean` to remove compiled PDF files
- Use `noter status` to check your setup

Happy note-taking! 📚
"#,
        config.paths.notes_dir,
        config.paths.obsidian_dir,
        config.paths.templates_dir,
        crate::config::Config::config_file_path()
            .unwrap_or_default()
            .display(),
        config.author,
        config.template_version,
        config.paths.templates_dir
    );

    fs::write("README.md", readme_content)?;
    println!("  Created: {}", "README.md".dimmed());

    Ok(())
}

fn create_gitignore(config: &crate::config::Config) -> Result<()> {
    let gitignore_content = format!(
        r#"# Compiled PDFs (uncomment to ignore PDFs)
# *.pdf

# Typst cache
.typst-cache/

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Editor files
.vscode/
.idea/
*.swp
*.swo
*~

# Temporary files
*.tmp
*.temp

# Backup files
*.bak
*.backup

# Log files
*.log

# Configuration backup (keep main config)
{}/config.json.bak
"#,
        crate::config::Config::config_dir()
            .unwrap_or_default()
            .display()
    );

    fs::write(".gitignore", gitignore_content)?;
    println!("  Created: {}", ".gitignore".dimmed());

    Ok(())
}

pub fn clean_setup() -> Result<()> {
    let config = get_config()?;

    println!(
        "{} This will remove all directories and files created by setup.",
        "⚠️".yellow()
    );
    println!("The following will be deleted:");
    println!("  • {}", config.paths.notes_dir);
    println!("  • {}", config.paths.obsidian_dir);
    println!("  • {}", config.paths.templates_dir);
    println!("  • README.md");
    println!("  • .gitignore");
    println!();

    print!("Are you sure? Type 'yes' to confirm: ");
    use std::io::{self, Write};
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "yes" {
        println!("Cancelled.");
        return Ok(());
    }

    // Remove directories
    let dirs_to_remove = [
        &config.paths.notes_dir,
        &config.paths.obsidian_dir,
        &config.paths.templates_dir,
    ];

    for dir in dirs_to_remove {
        if std::path::Path::new(dir).exists() {
            fs::remove_dir_all(dir)?;
            println!("{} Removed: {}", "🗑️".red(), dir);
        }
    }

    // Remove files
    let files_to_remove = ["README.md", ".gitignore"];
    for file in files_to_remove {
        if std::path::Path::new(file).exists() {
            fs::remove_file(file)?;
            println!("{} Removed: {}", "🗑️".red(), file);
        }
    }

    println!();
    println!("{} Setup cleanup completed!", "✅".green());
    println!("Run {} to set up again.", "noter setup".bright_white());

    Ok(())
}
