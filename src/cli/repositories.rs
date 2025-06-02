use colored::*;
use std::process::{Command, Output};

#[derive(Debug, Clone, PartialEq)]
pub struct RepoInfo {
    pub url: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Repositories {
    repositories: Vec<RepoInfo>,
}

impl Repositories {
    pub fn new() -> Self {
        Repositories {
            repositories: vec![
                RepoInfo {
                    url: String::from("https://github.com/epishev-m/e314-exceptions.git"),
                    name: String::from("com.e314.exceptions"),
                    version: String::from("1.1.2"),
                },
                RepoInfo {
                    url: String::from("https://github.com/epishev-m/e314-protect.git"),
                    name: String::from("com.e314.protect"),
                    version: String::from("2.1.1"),
                },
            ]
        }
    }

    pub fn get_repo_by_name(&self, name: &str) -> Result<&RepoInfo, String> {
        self.repositories
            .iter()
            .find(|repo| repo.name == name)
            .ok_or_else(|| format!("{} Repository named '{}' was not found", "✗".red(), name))
    }

    pub fn get_repo_by_idx(&self, idx: usize) -> Result<&RepoInfo, String> {
        if idx >= self.repositories.len() {
            Err(format!("{} Idx {} is out of bounds", "✗".red(), idx))
        } else {
            Ok(&self.repositories[idx])
        }
    }

    pub fn list(&self) {
        eprintln!("Modules:");
        for (index, repo) in self.repositories.iter().enumerate() {
            println!("  [{}] {} - {} - {}", index, repo.name, repo.version, repo.url);
        }
    }

    pub fn clone_by_index(&self, index: usize) -> Result<(), String> {
        if index >= self.repositories.len() {
            return Err(format!("{} Index {} is out of repositories list bounds", "✗".red(), index));
        }

        let repo = &self.repositories[index];

        let output = Command::new("git")
            .args(["clone", &repo.url])
            .output()
            .map_err(|e| format!("Error running git: {}", e))?;

        get_result(output, repo)
    }

    pub fn clone_all(&self) -> Vec<Result<(), String>> {
        let mut results = Vec::new();

        for (_, repo) in self.repositories.iter().enumerate() {
            let output: std::io::Result<Output> = Command::new("git")
                .args(["clone", &repo.url])
                .output();

            let result = get_result(output.unwrap(), repo);
            results.push(result);
        }

        results
    }
}

fn get_result(output: Output, repo: &RepoInfo) -> Result<(), String> {
    let result: Result<(), String> = if output.status.success() {
        println!("  {} {}", "✓".green(), repo.name);
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("  {} {}\n    {}", "✗".red(), repo.name, error);
        Err(String::from(""))
    };
    result
}