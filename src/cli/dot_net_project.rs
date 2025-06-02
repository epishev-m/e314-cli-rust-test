use crate::cli::repositories::RepoInfo;
use chrono::{Datelike, Utc};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct DoNetProject {
    name: String,
    display_name: String,
    version: String,
    description: String,
    dependencies: Vec<RepoInfo>,
    author_name: String,
    author_email: String,
}

impl DoNetProject {
    pub fn new(name: String,
               display_name: String,
               version: String,
               description: String,
               author_name: String,
               author_email: String,
               dependencies: Vec<RepoInfo>) -> Self {
        println!("\n");

        DoNetProject {
            name,
            display_name,
            version,
            description,
            author_name,
            author_email,
            dependencies
        }
    }

    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(self.name.as_str());
        fs::create_dir_all(path)?;
        self.create_gitignore(path)?;
        self.create_license(path)?;
        self.create_readme(path)?;
        self.create_sln(path)?;
        self.create_project(path)?;
        self.create_project_test(path)?;
        Ok(())
    }

    fn create_gitignore(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let gitignore_path = path.join(".gitignore");
        let mut file = File::create(gitignore_path)?;
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

    fn create_license(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
        fs::write(path.join("LICENSE"), license_content)?;
        Ok(())
    }

    fn create_readme(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let readme_content = format!(
            "# {}\n\n\
            {}\n\n\
            ## Installation\n\n\
            - [Installation - RU](UPM/Documentation~/installation-ru.md)\n\
            - [Installation - EN](UPM/Documentation~/installation-en.md)\n\n\
            ## Instructions\n\n\
            - [Instructions - RU](UPM/Documentation~/instructions-ru.md)\n\
            - [Instructions - EN](UPM/Documentation~/instructions-en.md)\n",
            self.display_name, self.description
        );
        fs::write(path.join("README.md"), readme_content)?;
        Ok(())
    }

    fn create_sln(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let sln_content = format!(
            "\n\
            Microsoft Visual Studio Solution File, Format Version 12.00\n\
            Project(\"{{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}}\") = \"{0}\", \"{0}\\{0}.csproj\", \"{{15DD8961-A2DF-4A14-A904-AE9947DB32DD}}\"\n\
            EndProject\n\
            Project(\"{{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}}\") = \"{0}.Tests\", \"{0}.Tests\\{0}.Tests.csproj\", \"{{E6F73FAB-4EB1-45C2-B4BD-4070026748B9}}\"\n\
            EndProject\n\
            Global\n\
            	GlobalSection(SolutionConfigurationPlatforms) = preSolution\n\
            		Debug|Any CPU = Debug|Any CPU\n\
		            Release|Any CPU = Release|Any CPU\n\
            	EndGlobalSection\n\
	            GlobalSection(ProjectConfigurationPlatforms) = postSolution\n\
                    {{15DD8961-A2DF-4A14-A904-AE9947DB32DD}}.Debug|Any CPU.ActiveCfg = Debug|Any CPU\n\
                    {{15DD8961-A2DF-4A14-A904-AE9947DB32DD}}.Debug|Any CPU.Build.0 = Debug|Any CPU\n\
                    {{15DD8961-A2DF-4A14-A904-AE9947DB32DD}}.Release|Any CPU.ActiveCfg = Release|Any CPU\n\
                    {{15DD8961-A2DF-4A14-A904-AE9947DB32DD}}.Release|Any CPU.Build.0 = Release|Any CPU\n\
                    {{E6F73FAB-4EB1-45C2-B4BD-4070026748B9}}.Debug|Any CPU.ActiveCfg = Debug|Any CPU\n\
                    {{E6F73FAB-4EB1-45C2-B4BD-4070026748B9}}.Debug|Any CPU.Build.0 = Debug|Any CPU\n\
                    {{E6F73FAB-4EB1-45C2-B4BD-4070026748B9}}.Release|Any CPU.ActiveCfg = Release|Any CPU\n\
                    {{E6F73FAB-4EB1-45C2-B4BD-4070026748B9}}.Release|Any CPU.Build.0 = Release|Any CPU\n\
            	EndGlobalSection\n\
            EndGlobal\n",
            self.display_name
        );
        fs::write(path.join(format!("{}.sln", self.display_name)), sln_content)?;
        Ok(())
    }

    fn create_project(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let project_path = path.join(format!("{}", self.display_name));
        fs::create_dir_all(&project_path)?;
        let sln_content = format!(
            "<Project Sdk=\"Microsoft.NET.Sdk\">\n\n\
                <PropertyGroup>\n\
                    <TargetFramework>net6.0</TargetFramework>\n\
                    <Nullable>disable</Nullable>\n\
                    <LangVersion>9</LangVersion>\n\
                    <GeneratePackageOnBuild>true</GeneratePackageOnBuild>\n\
                    <Title>{0}</Title>\n\
                    <Authors>{1} ({2})</Authors>\n\
                    <Copyright>Copyright (c) 2025 {1} ({2})</Copyright>\n\
                    <RepositoryType>Git</RepositoryType>\n\
                    <Version>{3}</Version>\n\
                    <PackageId>{0}</PackageId>\n\
                    <RepositoryUrl>https://github.com/</RepositoryUrl>\n\
                    <Description>The {0} module provides methods for validating input data and conditions. It prevents errors caused by invalid data by throwing exceptions with informative messages.</Description>\n\
                    <PackageProjectUrl>https://github.com/</PackageProjectUrl>\n\
                    <PackageLicenseFile>LICENSE</PackageLicenseFile>\n\
                    <PackageReadmeFile>README.md</PackageReadmeFile>\n\
                    <PackageTags>contracts requires args arg unity</PackageTags>\n\
                    <PackageReleaseNotes>https://github.com/.../blob/master/{0}.Upm/Packages/.../CHANGELOG.md</PackageReleaseNotes>\n\
                </PropertyGroup>\n\n\
                <ItemGroup>\n\
                    <None Include=\"..\\UPM\\README.md\" Pack=\"true\" PackagePath=\"\\\" />\n\
                    <None Include=\"..\\LICENSE\" Pack=\"true\" PackagePath=\"\\\" />\n\
                </ItemGroup>\n\n\
            </Project>\n",
            self.display_name, self.author_name, self.author_email, self.version
        );
        fs::write(project_path.join(format!("{}.csproj", self.display_name)), sln_content)?;
        Ok(())
    }

    fn create_project_test(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let project_test_path = path.join(format!("{}.Tests", self.display_name));
        fs::create_dir_all(&project_test_path)?;
        let sln_content = format!(
            "<Project Sdk=\"Microsoft.NET.Sdk\">\n\n\
                <PropertyGroup>\n\
                    <OutputType>Exe</OutputType>\n\
                    <TargetFramework>net6.0</TargetFramework>\n\
                    <Nullable>disable</Nullable>\n\
                    <LangVersion>9</LangVersion>\n\
                </PropertyGroup>\n\n\
                <ItemGroup>\n\
                    <PackageReference Include=\"Microsoft.NET.Test.Sdk\" Version=\"17.13.0\" />\n\
                    <PackageReference Include=\"NUnit\" Version=\"4.1.0\" />\n\
                </ItemGroup>\n\n\
                <ItemGroup>\n\
                  <ProjectReference Include=\"..\\{0}\\{0}.csproj\" />\n\
                </ItemGroup>\n\n\
            </Project>\n",
            self.display_name
        );
        fs::write(project_test_path.join(format!("{}.Tests.csproj", self.display_name)), sln_content)?;
        Ok(())
    }
}