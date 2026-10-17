use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub primary_language: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    pub default_tier: String, // "rpc", "embed", "ffi"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpmManifest {
    pub project: ProjectConfig,
    #[serde(default)]
    pub ecosystems: Vec<String>,
    #[serde(default)]
    pub foreign_dependencies: BTreeMap<String, String>,
    #[serde(default)]
    pub transports: Option<TransportConfig>,
    #[serde(default)]
    pub scripts: BTreeMap<String, String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

impl UpmManifest {
    pub fn new(name: &str, version: &str, primary_lang: Option<&str>, ecosystems: Vec<String>) -> Self {
        Self {
            project: ProjectConfig {
                name: name.to_string(),
                version: version.to_string(),
                primary_language: primary_lang.map(|s| s.to_string()),
                authors: Vec::new(),
            },
            ecosystems,
            foreign_dependencies: BTreeMap::new(),
            transports: Some(TransportConfig {
                default_tier: "rpc".to_string(),
            }),
            scripts: BTreeMap::new(),
            env: BTreeMap::new(),
        }
    }

    pub fn default_for(name: &str, ecosystems: Vec<String>) -> Self {
        Self::new(name, "0.1.0", None, ecosystems)
    }

    pub fn load_from_dir(path: &Path) -> Option<Self> {
        Self::try_load_from_dir(path).unwrap_or_else(|e| {
            eprintln!("  ⚠ Warning: Could not parse manifest (cpm.toml/upm.toml): {}", e);
            None
        })
    }

    pub fn try_load_from_dir(path: &Path) -> anyhow::Result<Option<Self>> {
        let cpm_path = path.join("cpm.toml");
        let manifest_path = if cpm_path.exists() {
            cpm_path
        } else {
            path.join("upm.toml")
        };
        if manifest_path.exists() {
            let content = fs::read_to_string(&manifest_path)?;
            let manifest: UpmManifest = toml::from_str(&content)?;
            Ok(Some(manifest))
        } else {
            Ok(None)
        }
    }

    pub fn save_to_dir(&self, path: &Path) -> anyhow::Result<()> {
        fs::create_dir_all(path)?;
        let manifest_name = if path.join("cpm.toml").exists() {
            "cpm.toml"
        } else {
            "upm.toml"
        };
        let manifest_path = path.join(manifest_name);
        let content = toml::to_string_pretty(self)?;
        let tmp_path = path.join(format!(".{}.tmp", manifest_name));

        // Atomic file write with safe replacement on Windows
        if fs::write(&tmp_path, &content).is_ok() {
            if manifest_path.exists() {
                let _ = fs::remove_file(&manifest_path);
            }
            if fs::rename(&tmp_path, &manifest_path).is_err() {
                let _ = fs::write(&manifest_path, &content);
                let _ = fs::remove_file(&tmp_path);
            }
        } else {
            fs::write(&manifest_path, content)?;
        }
        Ok(())
    }

    pub fn add_foreign_dependency(&mut self, eco: &str, pkg: &str, version: &str) {
        let key = format!("{}:{}", eco, pkg);
        self.foreign_dependencies.insert(key, version.to_string());
    }

    pub fn remove_foreign_dependency(&mut self, eco: &str, pkg: &str) -> bool {
        let key = format!("{}:{}", eco, pkg);
        self.foreign_dependencies.remove(&key).is_some()
    }

    pub fn has_ecosystem(&self, eco: &str) -> bool {
        self.ecosystems.iter().any(|e| e.eq_ignore_ascii_case(eco))
    }

    pub fn bootstrap_native_manifests(path: &Path, name: &str, languages: &[String]) -> anyhow::Result<()> {
        for lang in languages {
            let normalized = lang.to_lowercase();
            match normalized.as_str() {
                "javascript" | "typescript" | "node" | "pnpm" | "npm" | "yarn" | "bun" => {
                    let pkg_json = path.join("package.json");
                    if !pkg_json.exists() {
                        let content = format!(
                            "{{\n  \"name\": \"{}\",\n  \"version\": \"0.1.0\",\n  \"private\": true,\n  \"dependencies\": {{}}\n}}\n",
                            name
                        );
                        fs::write(pkg_json, content)?;
                    }
                    let index_js = path.join("index.js");
                    if !index_js.exists() {
                        let _ = fs::write(index_js, "// Polyglot Node.js Entrypoint\nconsole.log('CPM Node.js App');\n");
                    }
                }
                "python" | "pip" | "uv" | "poetry" => {
                    let pyproject = path.join("pyproject.toml");
                    if !pyproject.exists() {
                        let content = format!(
                            "[project]\nname = \"{}\"\nversion = \"0.1.0\"\ndescription = \"CPM polyglot project\"\ndependencies = []\n",
                            name
                        );
                        fs::write(pyproject, content)?;
                    }
                    let main_py = path.join("main.py");
                    if !main_py.exists() {
                        let _ = fs::write(main_py, "# Polyglot Python Entrypoint\nprint('CPM Python App')\n");
                    }
                }
                "rust" | "cargo" => {
                    let cargo_toml = path.join("Cargo.toml");
                    if !cargo_toml.exists() {
                        let content = format!(
                            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
                            name
                        );
                        fs::write(cargo_toml, content)?;
                    }
                    let src_dir = path.join("src");
                    let _ = fs::create_dir_all(&src_dir);
                    let main_rs = src_dir.join("main.rs");
                    if !main_rs.exists() {
                        let _ = fs::write(main_rs, "fn main() {\n    println!(\"CPM Rust App\");\n}\n");
                    }
                }
                "go" => {
                    let go_mod = path.join("go.mod");
                    if !go_mod.exists() {
                        let content = format!("module {}\n\ngo 1.21\n", name);
                        fs::write(go_mod, content)?;
                    }
                    let main_go = path.join("main.go");
                    if !main_go.exists() {
                        let _ = fs::write(main_go, "package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"CPM Go App\")\n}\n");
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
