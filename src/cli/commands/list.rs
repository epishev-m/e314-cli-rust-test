use crate::cli::repositories::Repositories;
use std::error;

pub fn execute() -> Result<(), Box<dyn error::Error>> {
    let framework = Repositories::new();
    eprintln!("\n");
    framework.list();
    Ok(())
}