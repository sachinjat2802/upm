use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub primary_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    pub default_tier: String, // "rpc", "embed", "ffi"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpmManifest {
    pub project: ProjectConfig,
    pub ecosystems: Vec<String>,
    #[serde(default)]
    pub foreign_dependencies: BTreeMap<String, String>,
    #[serde(default)]
    pub transports: Option<TransportConfig>,
}

impl UpmManifest {
    pub fn new(name: &str, version: &str, primary_lang: Option<&str>, ecosystems: Vec<String>) -> Self {
        let mut foreign_deps = BTreeMap::new();
        for eco in &ecosystems {
            match eco.as_str() {
                "node" | "pnpm" | "npm" | "yarn" | "bun" => {
                    if primary_lang != Some("javascript") && primary_lang != Some("typescript") && primary_lang != Some("node") {
                        foreign_deps.insert("npm:express".into(), "^4.19.2".into());
                    }
                }
                "python" | "pip" | "uv" | "poetry" => {
                    if primary_lang != Some("python") {
                        foreign_deps.insert("pip:docling".into(), "^2.0.0".into());
                    }
                }
                "rust" | "cargo" => {
                    if primary_lang != Some("rust") {
                        foreign_deps.insert("cargo:serde".into(), "^1.0".into());
                    }
                }
                _ => {}
            }
        }

        Self {
            project: ProjectConfig {
                name: name.to_string(),
                version: version.to_string(),
                primary_language: primary_lang.map(|s| s.to_string()),
            },
            ecosystems,
            foreign_dependencies: foreign_deps,
            transports: Some(TransportConfig {
                default_tier: "rpc".to_string(),
            }),
        }
    }

    pub fn default_for(name: &str, ecosystems: Vec<String>) -> Self {
        Self::new(name, "0.1.0", Some("javascript"), ecosystems)
    }

    pub fn load_from_dir(path: &Path) -> Option<Self> {
        let manifest_path = path.join("upm.toml");
        if manifest_path.exists() {
            if let Ok(content) = fs::read_to_string(&manifest_path) {
                if let Ok(manifest) = toml::from_str(&content) {
                    return Some(manifest);
                }
            }
        }
        None
    }

    pub fn save_to_dir(&self, path: &Path) -> anyhow::Result<()> {
        let manifest_path = path.join("upm.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(manifest_path, content)?;
        Ok(())
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
                        let _ = fs::write(index_js, "// Polyglot Node.js Entrypoint\nconsole.log('UPM Node.js App');\n");
                    }
                }
                "python" | "pip" | "uv" | "poetry" => {
                    let pyproject = path.join("pyproject.toml");
                    if !pyproject.exists() {
                        let content = format!(
                            "[project]\nname = \"{}\"\nversion = \"0.1.0\"\ndescription = \"Universal Package Platform polyglot project\"\ndependencies = []\n",
                            name
                        );
                        fs::write(pyproject, content)?;
                    }
                    let main_py = path.join("main.py");
                    if !main_py.exists() {
                        let _ = fs::write(main_py, "# Polyglot Python Entrypoint\nprint('UPM Python App')\n");
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
                        let _ = fs::write(main_rs, "fn main() {\n    println!(\"UPM Rust App\");\n}\n");
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
                        let _ = fs::write(main_go, "package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"UPM Go App\")\n}\n");
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
