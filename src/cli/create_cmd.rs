//! # Agentic App Creation Engine
//!
//! Radically reduces software development complexity by scaffolding complete,
//! production-grade polyglot and agentic AI applications in 1 command.

use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

/// Execute the `cpm create` / `cpm new` command.
pub fn execute_create(target_dir: &Path, template: Option<&str>, app_name: Option<&str>) -> Result<()> {
    let raw_name = app_name
        .map(|s| s.to_string())
        .or_else(|| {
            target_dir
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "agentic-app".to_string());

    let template_type = template.unwrap_or("agentic").to_lowercase();

    println!();
    println!("  {}", "🚀 CPM Agentic App Creation Engine".bold().cyan());
    println!("  {}", "═".repeat(60).dimmed());
    println!("  App Name:       {}", raw_name.bold().yellow());
    println!("  Template:       {}", template_type.bold().green());
    println!("  Destination:    {}", target_dir.display().to_string().cyan());
    println!();

    fs::create_dir_all(target_dir)?;

    match template_type.as_str() {
        "agentic" | "agent" | "ai" => scaffold_agentic_app(target_dir, &raw_name)?,
        "langgraph" | "graph" => scaffold_langgraph_app(target_dir, &raw_name)?,
        "langchain" | "chain" => scaffold_langchain_app(target_dir, &raw_name)?,
        "multiagent" | "team" | "autogen" => scaffold_multiagent_app(target_dir, &raw_name)?,
        "polyglot" | "polyglot-ai" => scaffold_polyglot_ai_app(target_dir, &raw_name)?,
        "fullstack" | "web" => scaffold_fullstack_app(target_dir, &raw_name)?,
        "cli" => scaffold_cli_app(target_dir, &raw_name)?,
        other => {
            println!("  {} Unknown template '{}'. Defaulting to 'agentic'.", "!".yellow(), other);
            scaffold_agentic_app(target_dir, &raw_name)?;
        }
    }

    // Auto-create AI agent guidelines
    crate::cli::ai_cmd::execute_ai(target_dir, Some("sync"), None, None)?;

    println!();
    println!("  {} Application created with zero boilerplate!", "🎉".bold().green());
    println!();
    println!("  {}", "Next steps:".bold().white());
    println!("    1. cd {}", target_dir.display().to_string().cyan());
    println!("    2. {}   (installs all polyglot dependencies)", "cpm install".bold().green());
    println!("    3. {}       (starts unified dev environment)", "cpm run dev".bold().green());
    println!();
    println!("  {} AI Agents can immediately understand this repo using: {}", "🤖".cyan(), "cpm agent context".bold().yellow());
    println!();

    Ok(())
}

fn scaffold_agentic_app(dir: &Path, name: &str) -> Result<()> {
    // 1. upm.toml
    let upm_toml = format!(
        r#"ecosystems = ["python", "node"]

[project]
name = "{name}"
version = "0.1.0"
authors = ["Human & AI Agent"]

[foreign_dependencies]
"pip:fastapi" = "^0.110.0"
"pip:uvicorn" = "^0.28.0"
"pip:pydantic" = "^2.6.0"
"npm:dotenv" = "^16.4.5"

[scripts]
dev = "python -m uvicorn server:app --reload --port 8000"
start = "python -m uvicorn server:app --port 8000"
test = "python -m unittest discover"
lint = "cpm audit"

[env]
PORT = "8000"
CPM_ENV = "development"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;

    // 2. Python backend server.py
    let server_py = format!(
        r#"""
{name} — Agentic AI Microservice
Managed by CPM / UPM Platform
"""
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import Dict, Any, List

app = FastAPI(
    title="{name} Agentic API",
    description="Universal AI agent runtime powered by CPM",
    version="0.1.0"
)

class AgentTask(BaseModel):
    prompt: str
    context: Dict[str, Any] = {{}}

class AgentResponse(BaseModel):
    status: str
    result: str
    tokens_used: int = 42

@app.get("/")
def health_check():
    return {{"status": "healthy", "service": "{name}", "runtime": "CPM Polyglot Engine"}}

@app.post("/agent/execute", response_model=AgentResponse)
def execute_agent(task: AgentTask):
    if not task.prompt:
        raise HTTPException(status_code=400, detail="Prompt cannot be empty")
    return AgentResponse(
        status="completed",
        result=f"Agent processed prompt: '{{task.prompt}}' successfully with zero latency."
    )

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("server:app", host="0.0.0.0", port=8000, reload=True)
"#
    );
    fs::write(dir.join("server.py"), server_py)?;

    // 3. MCP server configuration
    let mcp_json = format!(
        r#"{{
  "mcpServers": {{
    "{name}-server": {{
      "command": "python",
      "args": ["server.py"],
      "env": {{
        "PYTHONUNBUFFERED": "1"
      }}
    }}
  }}
}}
"#
    );
    fs::write(dir.join("mcp.json"), mcp_json)?;

    // 4. README.md
    let readme = format!(
        r#"# {name}

> Built with the **CPM Agentic Era Development Kit**

## Architecture
- **Ecosystems:** Python (FastAPI) + Node.js (Micro-tools)
- **AI Agent Context:** Native `mcp.json` and universal IDE guidelines (`CLAUDE.md`, `.cursorrules`)
- **Storage:** High-capacity D: drive caching enabled

## Getting Started
```bash
# 1. Install dependencies
cpm install

# 2. Start development server
cpm run dev

# 3. Check AI agent context
cpm agent context
```
"#
    );
    fs::write(dir.join("README.md"), readme)?;

    Ok(())
}

fn scaffold_polyglot_ai_app(dir: &Path, name: &str) -> Result<()> {
    let upm_toml = format!(
        r#"ecosystems = ["cargo", "python"]

[project]
name = "{name}"
version = "0.1.0"

[foreign_dependencies]
"pip:numpy" = "^1.26.0"
"pip:scipy" = "^1.12.0"

[scripts]
dev = "cargo run"
test = "cargo test"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;

    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let main_rs = r#"fn main() {
    println!("🚀 Polyglot AI Application active.");
    println!("Call Python AI inference from Rust with zero overhead via CPM Bridge!");
}
"#;
    fs::write(src_dir.join("main.rs"), main_rs)?;
    Ok(())
}

fn scaffold_fullstack_app(dir: &Path, name: &str) -> Result<()> {
    let upm_toml = format!(
        r#"ecosystems = ["python", "node"]

[project]
name = "{name}"
version = "0.1.0"

[scripts]
dev = "cpm run start"
start = "python -m http.server 3000"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;

    let index_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>{name}</title>
  <style>
    body {{ font-family: system-ui, sans-serif; background: #0b0f19; color: #f3f4f6; display: flex; height: 100vh; align-items: center; justify-content: center; margin: 0; }}
    .card {{ background: #1f2937; padding: 2.5rem; border-radius: 1rem; border: 1px solid #374151; text-align: center; max-width: 480px; box-shadow: 0 20px 25px -5px rgba(0,0,0,0.5); }}
    h1 {{ color: #38bdf8; margin-top: 0; }}
    code {{ background: #111827; padding: 0.25rem 0.5rem; border-radius: 0.25rem; color: #a5b4fc; }}
  </style>
</head>
<body>
  <div class="card">
    <h1>🚀 {name}</h1>
    <p>Your Agentic Fullstack Application is running with <code>cpm</code>!</p>
  </div>
</body>
</html>
"#
    );
    fs::write(dir.join("index.html"), index_html)?;
    Ok(())
}

fn scaffold_cli_app(dir: &Path, name: &str) -> Result<()> {
    let upm_toml = format!(
        r#"ecosystems = ["cargo"]

[project]
name = "{name}"
version = "0.1.0"

[scripts]
run = "cargo run --"
build = "cargo build --release"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;

    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir)?;
    let main_rs = r#"fn main() {
    println!("Hello from CPM high-speed CLI tool!");
}
"#;
    fs::write(src_dir.join("main.rs"), main_rs)?;
    Ok(())
}

fn scaffold_langgraph_app(dir: &Path, name: &str) -> Result<()> {
    let upm_toml = format!(
        r#"ecosystems = ["python"]

[project]
name = "{name}"
version = "0.1.0"

[foreign_dependencies]
"pip:langgraph" = "^0.2.0"
"pip:langchain-core" = "^0.3.0"
"pip:pydantic" = "^2.6.0"

[scripts]
dev = "python agent_graph.py"
start = "python agent_graph.py"
test = "cpm test"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;
    crate::cli::agent_cmd::execute_agent(dir, Some("graph"), false)?;
    Ok(())
}

fn scaffold_langchain_app(dir: &Path, name: &str) -> Result<()> {
    let upm_toml = format!(
        r#"ecosystems = ["python"]

[project]
name = "{name}"
version = "0.1.0"

[foreign_dependencies]
"pip:langchain" = "^0.3.0"
"pip:langchain-core" = "^0.3.0"

[scripts]
dev = "python agent_chain.py"
start = "python agent_chain.py"
test = "cpm test"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;
    crate::cli::agent_cmd::execute_agent(dir, Some("chain"), false)?;
    Ok(())
}

fn scaffold_multiagent_app(dir: &Path, name: &str) -> Result<()> {
    let upm_toml = format!(
        r#"ecosystems = ["python"]

[project]
name = "{name}"
version = "0.1.0"

[foreign_dependencies]
"pip:pydantic" = "^2.6.0"

[scripts]
dev = "python agent_team.py"
start = "python agent_team.py"
test = "cpm test"
"#
    );
    fs::write(dir.join("upm.toml"), upm_toml)?;
    crate::cli::agent_cmd::execute_agent(dir, Some("team"), false)?;
    crate::cli::agent_cmd::execute_agent(dir, Some("models"), false)?;
    Ok(())
}
