use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{Datelike, Utc};
use serde_json::{json, to_string_pretty};
use crate::cli::repositories::RepoInfo;

pub struct UPM {
    name: String,
    display_name: String,
    version: String,
    description: String,
    dependencies: Vec<RepoInfo>,
    author_name: String,
    author_email: String,
}

impl UPM {
    pub fn new(
        name: String,
        display_name: String,
        version: String,
        description: String,
        author_name: String,
        author_email: String,
        dependencies: Vec<RepoInfo>) -> Self {

        UPM {
            name,
            display_name,
            version,
            description,
            author_name,
            author_email,
            dependencies,
        }
    }

    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(self.name.as_str());
        let upm_path = path.join("UPM".to_string());
        fs::create_dir_all(&upm_path)?;
        self.create_package(&upm_path)?;
        self.create_license(&upm_path)?;
        self.create_changelog(&upm_path)?;
        self.create_readme(&upm_path)?;
        self.create_runtime(&upm_path)?;
        self.create_editor(&upm_path)?;
        self.create_tests(&upm_path)?;
        self.create_documentation(&upm_path)?;
        Ok(())
    }

    fn create_package(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut dependencies: HashMap<String, String> = HashMap::new();
        self.dependencies.iter().for_each(|dep| {
            dependencies.insert(dep.name.clone(), dep.version.clone());
        });

        let package_json = json!({
            "name": self.name,
            "displayName": self.display_name,
            "version": self.version,
            "author": {
                "name": self.author_name,
                "email": self.author_email
            },
            "description": self.description,
            "dependencies": dependencies
        });

        let package_content = to_string_pretty(&package_json)?;
        fs::write(path.join("package.json"), package_content)?;
        Ok(())
    }

    fn create_license(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let license_content = format!(
            "MIT License\n\n\
            Copyright (c) {} {} ({})\n\n\
            Permission is hereby granted, free of charge, to any person obtaining a copy\n\
            of this software and associated documentation files (the \"Software\"), to deal\n\
            in the Software without restriction, including without limitation the rights\n\
            to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\
            copies of the Software, and to permit persons to whom the Software is\n\
            furnished to do so, subject to the following conditions:\n\n\
            The above copyright notice and this permission notice shall be included in all\n\
            copies or substantial portions of the Software.\n\n\
            THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\
            IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\
            FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n\
            AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n\
            LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n\
            OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n\
            SOFTWARE.\n",
            Utc::now().year(), self.author_name, self.author_email
        );
        fs::write(path.join("LICENSE.md"), license_content)?;
        Ok(())
    }
    
    fn create_changelog(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let changelog_content = format!(
            "# Changelog\n\n\
            All notable changes to this project will be documented in this file.\n\n\
            The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),\n\
            and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n\
            ## [{}]\n\n\
            ### Added\n\n\
            - Initial release\n",
            self.version
        );
        fs::write(path.join("CHANGELOG.md"), changelog_content)?;
        Ok(())
    }
    
    fn create_readme(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let readme_content = format!(
            "# {}\n\n\
            {}\n\n\
            ## Installation\n\n\
            - [Installation - RU](Documentation~/installation-ru.md)\n\
            - [Installation - EN](Documentation~/installation-en.md)\n\n\
            ## Instructions\n\n\
            - [Instructions - RU](Documentation~/instructions-ru.md)\n\
            - [Instructions - EN](Documentation~/instructions-en.md)\n",
            self.display_name, self.description
        );
        fs::write(path.join("README.md"), readme_content)?;
        Ok(())
    }

    fn create_runtime(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let runtime_path = path.join("Runtime");
        fs::create_dir_all(&runtime_path)?;

        let references: Vec<String> = self.dependencies.iter()
            .map(|dep| dep.name.clone())
            .collect();

        let asmdef_content = json!({
            "name": self.display_name,
            "rootNamespace": self.display_name,
            "references": references,
            "includePlatforms": [],
            "excludePlatforms": [],
            "allowUnsafeCode": false,
            "overrideReferences": false,
            "precompiledReferences": [],
            "autoReferenced": true,
            "defineConstraints": [],
            "versionDefines": [],
            "noEngineReferences": false
        });

        let asmdef_content = to_string_pretty(&asmdef_content)?;
        fs::write(runtime_path.join(format!("{}.asmdef", self.display_name)), asmdef_content)?;
        Ok(())
    }

    fn create_editor(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let runtime_path = path.join("Editor");
        fs::create_dir_all(&runtime_path)?;

        let asmdef_content = json!({
            "name": format!("{}.Editor", self.display_name),
            "rootNamespace": format!("{}.Editor", self.display_name),
            "references": [
                self.display_name
            ],
            "includePlatforms": [
                "Editor"
            ],
            "excludePlatforms": [],
            "allowUnsafeCode": false,
            "overrideReferences": false,
            "precompiledReferences": [],
            "autoReferenced": true,
            "defineConstraints": [],
            "versionDefines": [],
            "noEngineReferences": false
        });

        let asmdef_content = to_string_pretty(&asmdef_content)?;
        fs::write(runtime_path.join(format!("{}Editor.asmdef", self.display_name)), asmdef_content)?;
        Ok(())
    }

    fn create_tests(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let tests_path = path.join("Tests");
        fs::create_dir_all(&tests_path)?;

        let asmdef_content = json!({
            "name": format!("{}.Tests", self.display_name),
            "rootNamespace": format!("{}.Tests", self.display_name),
            "references": [
                self.display_name
            ],
            "includePlatforms": [
                "Editor"
            ],
            "excludePlatforms": [],
            "allowUnsafeCode": false,
            "overrideReferences": false,
            "precompiledReferences": [],
            "autoReferenced": true,
            "defineConstraints": [],
            "versionDefines": [],
            "noEngineReferences": false
        });

        let asmdef_content = to_string_pretty(&asmdef_content)?;
        fs::write(tests_path.join(format!("{}.Tests.asmdef", self.display_name)), asmdef_content)?;
        Ok(())
    }

    fn create_documentation(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let doc_path = path.join("Documentation~");
        fs::create_dir_all(&doc_path)?;
        self.create_index(&doc_path)?;
        self.create_installation_en(&doc_path)?;
        self.create_installation_ru(&doc_path)?;
        self.create_instructions_en(&doc_path)?;
        self.create_instructions_ru(&doc_path)?;
        Ok(())
    }
    
    fn create_index(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let index = format!(
            "# {}\n\n\
            {}\n\n\
            ## Installation\n\n\
            - [Installation - RU](installation-ru.md)\n\
            - [Installation - EN](installation-en.md)\n\n\
            ## Instructions\n\n\
            - [Instructions - RU](instructions-ru.md)\n\
            - [Instructions - EN](instructions-en.md)\n",
            self.display_name, self.description
        );
        fs::write(path.join("index.md"), index)?;
        Ok(())
    }

    fn create_installation_en(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let index = format!(
            "# Installation\n\n\
            ## Content tree\n\n\
            - [Installation](#installation)\n\
              - [Content tree](#content-tree)\n\
              - [Compatibility](#compatibility)\n\
              - [Unity Package Manager. Git URL](#unity-package-manager-git-url)\n\
              - [Unity Package Manager. OpenUPM](#unity-package-manager-openupm)\n\
              - [NuGet](#nuget)\n\n\
            ## Compatibility\n\n\
            - The module has been tested with Unity 2022.3 LTS and above.\n\
            - Compatible with .NET Standard 2.0 and above.\n\n\
            ## Unity Package Manager. Git URL\n\n\
            ```ps1\n\n\
            https://github.com/\n\
            ```\n\n\
            1. Open Window → Package Manager.\n\n\
            2. Click on + → Add package from git URL...\n\n\
            3. Enter the URL and click Add.\n\n\
            ### Unity Package Manager. OpenUPM\n\n\
            ```ps1\n\n\
            https://openupm.com/packages/{}.html\n\
            ```\n\n\
            1. Open Edit → Project Settings → Package Manager.\n\
            2. Register a new OpenUPM registry if it hasn't been done yet.\n\
            3. Add com.e314 to Scopes.\n\
            4. Click Apply.\n\
            5. Open Window → Package Manager.\n\
            6. Click on the + button → Add package by name...\n\
            7. Enter the Name `{}` and Version `{}`.\n\
            8. Click Add.\n\n\
            ## NuGet\n\n\
            ```ps1\n\
            https://www.nuget.org/packages/{}\n\
            ```\n\n\
            1. Open the command line.\n\
            2. Navigate to the directory containing the project file.\n\
            3. Run the command to install the NuGet package:\n\n\
            ```sh\n\
            dotnet add package {} -v {}\n\
            ```\n",
            self.name,
            self.name,
            self.version,
            self.display_name,
            self.display_name,
            self.version
        );
        
        fs::write(path.join("installation-en.md"), index)?;
        Ok(())
    }

    fn create_installation_ru(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let index = format!(
            "# Установка\n\n\
            ## Содержание\n\n\
            - [Установка](#установка)\n\
              - [Содержание](#содержание)\n\
              - [Совместимость](#совместимость)\n\
              - [Unity Package Manager. Git URL](#unity-package-manager-git-url)\n\
              - [Unity Package Manager. OpenUPM](#unity-package-manager-openupm)\n\
              - [NuGet](#nuget)\n\n\
            ## Совместимость\n\n\
            - Модуль протестирован с Unity 2022.3 LTS и выше.\n\
            - Совместим с .NET Standard 2.0 и выше.\n\n\
            ## Unity Package Manager. Git URL\n\n\
            ```ps1\n\n\
            https://github.com/\n\
            ```\n\n\
            1. Открыть Window → Package Manager.\n\n\
            2. Нажать на + → Add package from git URL...\n\n\
            3. Ввести url и нажать Add.\n\n\
            ### Unity Package Manager. OpenUPM\n\n\
            ```ps1\n\n\
            https://openupm.com/packages/{}.html\n\
            ```\n\n\
            1. Открыть Edit → Project Settings → Package Manager.\n\
            2. Зарегистрировать новый реестр OpenUPM, если это еще не сделано.\n\
            3. Добавьте `com.e314` в  Scopes.\n\
            4. Нажать Apply.\n\
            5. Открыть Window → Package Manager.\n\
            6. Нажать на + button → Add package by name...\n\
            7. Введите Имя `{}` и Версию `{}`.\n\
            8. Нажмите Add.\n\n\
            ## NuGet\n\n\
            ```ps1\n\
            https://www.nuget.org/packages/{}\n\
            ```\n\n\
            1. Открыть командную строку.\n\
            2. Перейти в каталог, в котором находится файл проекта.\n\
            3. Выполнить команду для установки пакета NuGet:\n\n\
            ```sh\n\
            dotnet add package {} -v {}\n\
            ```\n",
            self.name,
            self.name,
            self.version,
            self.display_name,
            self.display_name,
            self.version
        );

        fs::write(path.join("installation-ru.md"), index)?;
        Ok(())
    }

    fn create_instructions_en(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let index = format!(
            "# {0}\n\n\
            ## Description\n\n\
            {1}\n",
            self.display_name, self.description
        );
        fs::write(path.join("instructions-en.md"), index)?;
        Ok(())
    }

    fn create_instructions_ru(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let index = format!(
            "# {0}\n\n\
            ## Описание\n\n\
            {1}\n",
            self.display_name, self.description
        );
        fs::write(path.join("instructions-en.md"), index)?;
        Ok(())
    }
}