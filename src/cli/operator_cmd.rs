/// # `cpm operator` CLI Subcommand — Kubernetes Operator CRD Manifest Generator
///
/// Auto-generates Kubernetes Custom Resource Definitions (CRD) and Custom Resources (CR)
/// tailored for deploying polyglot microservice clusters.

use colored::Colorize;
use std::path::Path;

/// Generate Kubernetes Operator CRD manifest.
pub fn execute_operator(path: &Path, out_file: Option<&str>) -> anyhow::Result<()> {
    println!();
    println!("  {}", "╭──────────────────────────────────────────────────────╮".cyan());
    println!("  {}  ☸️ {}{}",
        "│".cyan(),
        "CPM Kubernetes Operator CRD Generator".bold().white(),
        "           │".cyan(),
    );
    println!("  {}", "╰──────────────────────────────────────────────────────╯".cyan());
    println!();

    let target_name = out_file.unwrap_or("cpm_operator_crd.yaml");
    let target_path = path.join(target_name);

    println!("  ▶ Generating Kubernetes Custom Resource Definition (cpm.io/v1alpha1)...");

    let crd_yaml = r#"apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: cpmworkspaces.cpm.io
spec:
  group: cpm.io
  versions:
    - name: v1alpha1
      served: true
      storage: true
      schema:
        openAPIV3Schema:
          type: object
          properties:
            spec:
              type: object
              properties:
                ecosystems:
                  type: array
                  items:
                    type: string
                replicas:
                  type: integer
  scope: Namespaced
  names:
    plural: cpmworkspaces
    singular: cpmworkspace
    kind: CpmWorkspace
    shortNames:
      - cpmws
"#;

    std::fs::write(&target_path, crd_yaml)?;

    println!();
    println!("  {} Kubernetes Operator CRD manifest created at {}", "✔".green(), target_name.bold().yellow());
    println!("  {} Apply with: {}", "ℹ".blue(), format!("kubectl apply -f {}", target_name).cyan());
    println!();

    Ok(())
}
