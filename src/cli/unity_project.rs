use crate::cli::packages_manifest::PackagesManifest;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs;

pub struct UnityProject {
    path: String,
    name: String
}

impl UnityProject {
    pub fn new(path: String, name: String) -> Self {
        println!("\n");

        UnityProject {
            path,
            name,
        }
    }

    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(self.path.as_str());
        let unity_path = path.join(format!("{}.Unity", self.name));
        self.create_assets(&unity_path)?;
        self.create_project_settings(&unity_path)?;
        self.create_packages(&unity_path)?;
        self.create_gitignore(&unity_path)?;
        Ok(())
    }

    fn create_assets(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let assets_path = path.join("Assets");
        fs::create_dir_all(assets_path)?;
        Ok(())
    }
    
    fn create_project_settings(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let project_settings_path = path.join("ProjectSettings");
        fs::create_dir_all(&project_settings_path)?;
        let project_version_path = project_settings_path.join("ProjectVersion.txt");
        let mut file = File::create(project_version_path)?;
        writeln!(file, "m_EditorVersion: 6000.1.4f1")?;
        writeln!(file, "m_EditorVersionWithRevision: 6000.1.4f1 (03270eb687c6)")?;
        Ok(())
    }

    fn create_packages(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let packages_path = path.join("Packages");
        fs::create_dir_all(&packages_path)?;
        let packages_manifest = PackagesManifest::new();
        packages_manifest.create_manifest(&packages_path)?;
        packages_manifest.create_packages_lock(&packages_path)?;
        Ok(())
    }

    fn create_gitignore(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let gitignore_path = path.join(".gitignore");
        let mut file = File::create(gitignore_path)?;
        writeln!(file, "# =============== #")?;
        writeln!(file, "# Unity generated #")?;
        writeln!(file, "# =============== #")?;
        writeln!(file, "[Ll]ibrary/")?;
        writeln!(file, "[Tt]emp/")?;
        writeln!(file, "[Oo]bj/")?;
        writeln!(file, "[Bb]uild/")?;
        writeln!(file, "[Bb]uilds/")?;
        writeln!(file, "[Ll]ogs/")?;
        writeln!(file, "[Uu]ser[Ss]ettings/")?;
        writeln!(file, "[Mm]emoryCaptures/")?;
        writeln!(file, "[Rr]ecordings/")?;
        writeln!(file, "")?;
        writeln!(file, "# ====================== #")?;
        writeln!(file, "# Visual Studio / Rider #")?;
        writeln!(file, "# ====================== #")?;
        writeln!(file, "ExportedObj/")?;
        writeln!(file, ".consulo/")?;
        writeln!(file, "*.csproj")?;
        writeln!(file, "*.unityproj")?;
        writeln!(file, "*.sln")?;
        writeln!(file, "*.suo")?;
        writeln!(file, "*.tmp")?;
        writeln!(file, "*.user")?;
        writeln!(file, "*.userprefs")?;
        writeln!(file, "*.pidb")?;
        writeln!(file, "*.booproj")?;
        writeln!(file, "*.svd")?;
        writeln!(file, "*.pdb")?;
        writeln!(file, "*.mdb")?;
        writeln!(file, "*.opendb")?;
        writeln!(file, "*.VC.db")?;
        writeln!(file, "*.idea/")?;
        writeln!(file, "*.vs/")?;
        writeln!(file, "*.vsconfig")?;
        writeln!(file, "*.DotSettings")?;
        writeln!(file, "*.DotSettings.user")?;
        writeln!(file, "# ====================== #")?;
        writeln!(file, "# OS generated #")?;
        writeln!(file, "# ====================== #")?;
        writeln!(file, ".DS_Store")?;
        writeln!(file, ".DS_Store?")?;
        writeln!(file, "._*")?;
        writeln!(file, ".Spotlight-V100")?;
        writeln!(file, ".Trashes")?;
        writeln!(file, "Icon?")?;
        writeln!(file, "ehthumbs.db")?;
        writeln!(file, "Thumbs.db")?;
        writeln!(file, "desktop.ini")?;
        Ok(())
    }
}