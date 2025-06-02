use crate::cli::project::Project;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut project = Project::new();
    project.configure();
    project.create()?;
    Ok(())
}

