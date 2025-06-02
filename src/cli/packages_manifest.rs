use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub version: String,
    pub depth: u32,
    pub source: String,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagesManifest {
    #[serde(flatten)]
    pub packages: HashMap<String, PackageInfo>,
}

impl PackagesManifest {
    pub fn new() -> Self {
        Self {
            packages: HashMap::from([
                ("com.unity.test-framework".to_string(),
                 PackageInfo {
                     version: "1.1.24".to_string(),
                     depth: 0,
                     source: "https://registry.npmjs.org".to_string(),
                     dependencies: Some(HashMap::from([
                         ("com.unity.ext.nunit".to_string(), "2.0.3".to_string()),
                         ("com.unity.modules.imgui".to_string(), "1.0.0".to_string()),
                         ("com.unity.modules.jsonserialize".to_string(), "1.0.0".to_string())
                     ])),
                 }),
                ("com.unity.ide.rider".to_string(),
                 PackageInfo {
                     version: "3.0.36".to_string(),
                     depth: 0,
                     source: "registry".to_string(),
                     dependencies: Some(HashMap::from([
                         ("com.unity.ext.nunit".to_string(), "1.0.6".to_string()),
                     ])),
                 }),
                ("com.unity.testtools.codecoverage".to_string(),
                 PackageInfo {
                     version: "1.2.6".to_string(),
                     depth: 0,
                     source: "registry".to_string(),
                     dependencies: Some(HashMap::from([
                         ("com.unity.test-framework".to_string(), "1.0.16".to_string()),
                         ("com.unity.settings-manager".to_string(), "1.0.1".to_string()),
                     ])),
                 }),
            ])
        }
    }

    pub fn create_packages_lock(&self, dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(dir).join("packages-lock.json");
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn create_manifest(&self, dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(dir).join("manifest.json");
        let mut dependencies = HashMap::new();

        for (name, info) in &self.packages {
            dependencies.insert(name.clone(), info.version.clone());
        }

        let manifest = serde_json::json!({
            "dependencies": dependencies
        });

        let json = serde_json::to_string_pretty(&manifest)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}