use crate::cli::repositories::{RepoInfo, Repositories};
use crate::cli::unity_project::UnityProject;
use colored::Colorize;
use std::io::Write;
use std::io;
use crate::cli::dot_net_project::DoNetProject;
use crate::cli::upm::UPM;

pub struct Project {
    name: String,
    display_name: String,
    version: String,
    description: String,
    dependencies: Vec<RepoInfo>,
    author_name: String,
    author_email: String,
    repositories: Repositories,
}

impl Project {
    pub fn new() -> Self {
        println!("\n");

        Project {
            name: String::new(),
            display_name: String::new(),
            version: String::new(),
            description: String::new(),
            dependencies: Vec::new(),
            author_name: String::new(),
            author_email: String::new(),
            repositories: Repositories::new(),
        }
    }
    
    pub fn configure(&mut self) {
        self.set_name();
        self.set_version();
        self.set_description();
        self.set_author_name();
        self.set_author_email();
        self.set_dependencies();
    }

    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_upm()?;
        self.create_unity_project()?;
        self.create_don_net_project()?;
        Ok(())
    }
    
    fn create_upm(&self) -> Result<(), Box<dyn std::error::Error>> {
        let upm = UPM::new(
            self.name.clone(),
            self.display_name.clone(),
            self.version.clone(),
            self.description.clone(),
            self.author_name.clone(),
            self.author_email.clone(),
            self.dependencies.clone());
        upm.create()?;
        Ok(())
    }
    
    fn create_unity_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        let uni_project = UnityProject::new(self.name.clone(), self.display_name.clone());
        uni_project.create()?;
        Ok(())
    }
    
    fn create_don_net_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dot_net_project = DoNetProject::new(
            self.name.clone(),
            self.display_name.clone(),
            self.version.clone(),
            self.description.clone(),
            self.author_name.clone(),
            self.author_email.clone(),
            self.dependencies.clone());
        dot_net_project.create()?;
        Ok(())
    }

    fn set_name(&mut self) {
        match read_input("Enter name: ") {
            Ok(input_name) => {
                if input_name.is_empty() {
                    println!("  {} Name - {}", "✗".red(), "empty".red());
                    self.set_name();
                    return;
                }
                let name = format!("com.e314.{}", input_name.to_lowercase());
                match self.repositories.get_repo_by_name(&name) {
                    Ok(_) => {
                        println!("  {} Name: {} - {}", "✗".red(), self.name, "already exists".red());
                        self.set_name();
                    },
                    Err(_) => {
                        self.name = name;
                        self.display_name = format!("E314.{}", input_name);
                        println!("  {} Name: {}", "✓".green(), self.name.blue());
                        println!("  {} Display Name: {}", "✓".green(), self.display_name.blue());
                    }
                }
            }
            Err(e) => {
                println!("  {} Name - {}", "✗".red(), e.to_string().red());
                self.set_name();
            }
        }
    }
    
    fn set_version(&mut self) {
        match read_input("Enter version: ") {
            Ok(version) => {
                if is_valid_semantic_version(&version) {
                    self.version = version.clone();
                    println!("  {} Version: {}", "✓".green(), self.version.blue());
                    return;
                }
                println!("  {} Version: {} - {}", "✗".red(), version, "invalid format".red());
                println!("    Format X.Y.Z (non-negative integers without leading zeros)");
                self.set_version();
            }
            Err(e) => {
                println!("  {} Version - {}", "✗".red(), e.to_string().red());
                self.set_version();
            }
        }
    }

    fn set_description(&mut self) {
        match read_input("Enter description: ") {
            Ok(input_description) => {
                if input_description.is_empty() {
                    println!("  {} Description - {}", "✗".red(), "empty".red());
                    return;
                }
                self.description = input_description.clone();
                println!("  {} Description: [...]", "✓".green());
            },
            Err(e) => {
                println!("  {} Description - {}", "✗".red(), e.to_string().red());
                self.set_description();
            }
        }
    }

    fn set_author_name(&mut self) {
        match read_input("Enter author name: ") {
            Ok(name) => {
                if name.is_empty() {
                    println!("  {} Author name - {}", "✗".red(), "empty".red());
                    self.set_author_name();
                    return;
                }
                self.author_name = name;
                println!("  {} Author name: {}", "✓".green(), self.author_name.blue());
            }
            Err(e) => {
                println!("  {} Author name - {}", "✗".red(), e.to_string().red());
                self.set_author_name();
            }
        }
    }
    
    fn set_author_email(&mut self) {
        match read_input("Enter author email: ") {
            Ok(email) => {
                if is_valid_email(&email) {
                    self.author_email = email;
                    println!("  {} Author email: {}", "✓".green(), self.author_email.blue());
                    return;
                }
                println!("  {} Author email - {}", "✗".red(), "invalid format".red());
                println!("    Format: name@example.com");
                self.set_author_email();
            }
            Err(e) => {
                println!("  {} Author email - {}", "✗".red(), e.to_string().red());
                self.set_author_email();
            }
        }
    }

    fn set_dependencies(&mut self) {
        self.repositories.list();
        match read_input("Enter idxs with a space: ") {
            Ok(idxs) => {
                let idx_nums: Vec<usize> = idxs.split(' ')
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect();

                for idx in idx_nums {
                    match self.repositories.get_repo_by_idx(idx) {
                        Ok(repo) => {
                            if !self.dependencies.contains(repo) {
                                self.dependencies.push(repo.clone());
                            }
                        }
                        Err(_) => println!("  {} Invalid idx: {}", "✗".red(), idx.to_string().red())
                    }
                }

                if self.dependencies.is_empty() {
                    println!("  {} Dependencies", "✓".green());
                } else {
                    println!("  {} Dependencies: {}", "✓".green(),
                             self.dependencies.iter()
                                 .map(|repo| repo.name.clone())
                                 .collect::<Vec<String>>()
                                 .join(", ")
                                 .blue());
                }
            }
            Err(e) => {
                println!("  {} Dependencies - {}", "✗".red(), e.to_string().red());
                self.set_dependencies();
            }
        }
    }
}

fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn is_valid_semantic_version(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false
    }
    for part in parts {
        if part.is_empty() || (part.starts_with('0') && part != "0") {
            return false
        }
        if !part.chars().all(|c| c.is_digit(10)) {
            return false
        }
    }
    true
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    email_regex.is_match(email)
}
