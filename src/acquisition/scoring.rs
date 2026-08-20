/// # Ecosystem Detection Scoring Engine
///
/// Scans a workspace directory and scores each registered ecosystem adapter
/// using five weighted signals:
///
/// | Signal              | Weight | Example                         |
/// |---------------------|--------|---------------------------------|
/// | Lockfile present    | +100   | `pnpm-lock.yaml` exists         |
/// | Manifest marker     | +80    | `[tool.uv]` inside pyproject    |
/// | Manifest present    | +40    | `package.json` exists           |
/// | Glob match          | +30    | `*.gemspec` matches             |
/// | Directory indicator | +20    | `node_modules/` exists          |
/// | Priority tie-break  | +0-4   | Adapter-defined precedence      |
///
/// The highest-scoring adapter per language group wins and is promoted
/// to `winner = true`. Multiple language groups can coexist in a polyglot
/// workspace (e.g. pnpm wins for JS, uv wins for Python).

use crate::acquisition::adapter::{AdapterRegistry, EcosystemAdapter};
use std::fs;
use std::path::{Path, PathBuf};

/// A single scored signal contributing to an ecosystem's total score.
#[derive(Debug, Clone)]
pub struct ScoreDetail {
    /// Human-readable signal name (e.g. "lockfile present").
    pub signal: String,
    /// Point weight of this signal.
    pub weight: u32,
    /// Descriptive example (e.g. "pnpm-lock.yaml present").
    pub example: String,
}

/// The combined score for a single ecosystem adapter in a workspace.
#[derive(Debug, Clone)]
pub struct EcosystemScore {
    /// The ecosystem adapter that was scored.
    pub adapter: EcosystemAdapter,
    /// Total accumulated score across all signals.
    pub total_score: u32,
    /// Breakdown of individual signal contributions.
    pub details: Vec<ScoreDetail>,
    /// Whether this adapter won its language group.
    pub winner: bool,
}

/// The full result of scanning a workspace directory.
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// Root path that was scanned.
    pub root_path: PathBuf,
    /// All scored ecosystems, sorted descending by score.
    pub scores: Vec<EcosystemScore>,
    /// Only the winning adapter per language group.
    pub detected_ecosystems: Vec<EcosystemAdapter>,
}

/// The scoring engine that scans a directory and produces detection results.
pub struct DetectionEngine {
    registry: AdapterRegistry,
}

impl DetectionEngine {
    /// Create a new engine with the given adapter registry.
    pub fn new(registry: AdapterRegistry) -> Self {
        Self { registry }
    }

    /// Scan a directory and produce scored detection results.
    ///
    /// Each registered adapter is evaluated against the filesystem.
    /// The highest-scoring adapter per language group is marked as `winner`.
    pub fn detect_dir(&self, path: &Path) -> DetectionResult {
        let mut scores = Vec::new();

        for adapter in self.registry.all() {
            let mut total = 0u32;
            let mut details = Vec::new();

            // Signal 1: Lockfile present (+100 pts)
            if let Some(ref lockfile) = adapter.lockfile_file {
                let lock_path = path.join(lockfile);
                if lock_path.exists() {
                    total += 100;
                    details.push(ScoreDetail {
                        signal: "lockfile present".into(),
                        weight: 100,
                        example: format!("{} present", lockfile),
                    });
                }
            }

            // Signal 3: Manifest file present (+40 pts)
            let manifest_path = path.join(&adapter.manifest_file);
            let mut manifest_content = None;
            if manifest_path.exists() {
                total += 40;
                details.push(ScoreDetail {
                    signal: "manifest present".into(),
                    weight: 40,
                    example: format!("{} present", adapter.manifest_file),
                });
                if let Ok(content) = fs::read_to_string(&manifest_path) {
                    manifest_content = Some(content);
                }
            }

            // Signal 2: Marker string inside manifest (+80 pts)
            if let (Some(content), Some(ref marker)) = (&manifest_content, &adapter.manifest_marker) {
                if content.contains(marker) {
                    total += 80;
                    details.push(ScoreDetail {
                        signal: "marker string inside manifest".into(),
                        weight: 80,
                        example: format!("found '{}' inside {}", marker, adapter.manifest_file),
                    });
                }
            }

            // Signal 4: Glob match (+30 pts)
            if let Some(ref glob_pattern) = adapter.glob_pattern {
                let pattern = path.join(glob_pattern);
                if let Ok(entries) = glob::glob(&pattern.to_string_lossy()) {
                    let count = entries.filter_map(Result::ok).count();
                    if count > 0 {
                        total += 30;
                        details.push(ScoreDetail {
                            signal: "glob match".into(),
                            weight: 30,
                            example: format!("matched {}", glob_pattern),
                        });
                    }
                }
            }

            // Signal 5: Directory present (+20 pts)
            if let Some(ref dir_name) = adapter.directory_indicator {
                let dir_path = path.join(dir_name);
                if dir_path.is_dir() {
                    total += 20;
                    details.push(ScoreDetail {
                        signal: "directory present".into(),
                        weight: 20,
                        example: format!("{}/ present", dir_name),
                    });
                }
            }

            // Signal 6: Declared priority (0–4 pts tie-break)
            if total > 0 {
                let prio = adapter.default_priority as u32;
                total += prio;
                details.push(ScoreDetail {
                    signal: "declared priority".into(),
                    weight: prio,
                    example: format!("tie-break priority {}", prio),
                });
            }

            scores.push(EcosystemScore {
                adapter: (*adapter).clone(),
                total_score: total,
                details,
                winner: false,
            });
        }

        // Determine highest score per language group
        let mut max_per_lang: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        for score in &scores {
            if score.total_score > 0 {
                let current_max = max_per_lang.entry(score.adapter.language.clone()).or_insert(0);
                if score.total_score > *current_max {
                    *current_max = score.total_score;
                }
            }
        }

        let mut detected_ecosystems = Vec::new();
        for score in &mut scores {
            if score.total_score > 0 {
                if let Some(&max_score) = max_per_lang.get(&score.adapter.language) {
                    if score.total_score == max_score {
                        score.winner = true;
                        detected_ecosystems.push(score.adapter.clone());
                    }
                }
            }
        }

        // Sort scores descending for display
        scores.sort_by(|a, b| b.total_score.cmp(&a.total_score));

        DetectionResult {
            root_path: path.to_path_buf(),
            scores,
            detected_ecosystems,
        }
    }
}
