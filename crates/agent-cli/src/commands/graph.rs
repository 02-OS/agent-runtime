use agent_core::{contain::write_private_atomic, AgentError, Result};
use agent_git::GitDiscovery;
use agent_index::GraphExportFormat;
use agent_memory::{export_graph, RepoHandle};
use serde_json::json;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn execute_export(
    format: &str,
    dataset: Option<String>,
    output: Option<PathBuf>,
    json_output: bool,
) -> Result<()> {
    let parsed = GraphExportFormat::parse(format)?;
    let current = env::current_dir()?;
    let root = GitDiscovery::find_repository_root(&current)?;
    let handle = RepoHandle::open(&root)?;
    let document = export_graph(&handle, dataset.as_deref(), parsed)?;

    if let Some(path) = output {
        if fs::symlink_metadata(&path).is_ok_and(|meta| meta.file_type().is_symlink()) {
            return Err(AgentError::Security(format!(
                "refusing to write graph export through symlink {}",
                path.display()
            )));
        }
        write_private_atomic(&path, document.as_bytes())?;
        if json_output {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "status": "success",
                    "format": format,
                    "path": path,
                }))?
            );
        } else {
            println!("Wrote {} graph to {}", format, path.display());
        }
    } else {
        println!("{document}");
    }
    Ok(())
}
