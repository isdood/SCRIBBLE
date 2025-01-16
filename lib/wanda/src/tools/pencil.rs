/// Wanda AI Cargo Project Editor Tool
/// Last Updated: 2025-01-16 04:05:54 UTC
/// Author: isdood
/// Current User: isdood

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use crate::scribble::errors::ScribbleError;
use crate::quantum::coherence::QuantumCoherence;

#[derive(Debug, Clone)]
pub struct EditOperation {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column: usize,
    pub old_text: String,
    pub new_text: String,
    pub quantum_signature: [u8; 32],
}

#[derive(Debug)]
pub struct CargoEditor {
    project_root: PathBuf,
    edit_history: Vec<EditOperation>,
    coherence: QuantumCoherence,
    backup_path: PathBuf,
}

impl CargoEditor {
    pub fn new<P: AsRef<Path>>(project_path: P) -> Result<Self, ScribbleError> {
        let root = project_path.as_ref().to_path_buf();
        if !root.join("Cargo.toml").exists() {
            return Err(ScribbleError::ProjectNotFound);
        }

        Ok(Self {
            project_root: root.clone(),
           edit_history: Vec::new(),
           coherence: QuantumCoherence::new(),
           backup_path: root.join(".wanda_backup"),
        })
    }

    pub fn apply_edit(&mut self, op: EditOperation) -> Result<(), ScribbleError> {
        if !self.coherence.is_stable() {
            return Err(ScribbleError::QuantumDecoherence);
        }

        let file_path = self.project_root.join(&op.file_path);
        if !file_path.exists() {
            return Err(ScribbleError::FileNotFound);
        }

        // Create backup before editing
        self.create_backup(&file_path)?;

        // Read file content
        let mut content = String::new();
        File::open(&file_path)?.read_to_string(&mut content)?;

        // Apply the edit
        let lines: Vec<&str> = content.lines().collect();
        let mut new_content = String::new();

        for (i, line) in lines.iter().enumerate() {
            if i + 1 == op.line_number {
                let before = &line[..op.column];
                let after = &line[op.column + op.old_text.len()..];
                new_content.push_str(&format!("{}{}{}\n", before, op.new_text, after));
            } else {
                new_content.push_str(line);
                new_content.push('\n');
            }
        }

        // Write changes
        fs::write(&file_path, new_content)?;
        self.edit_history.push(op);
        self.coherence.update();

        Ok(())
    }

    pub fn compile_project(&self) -> Result<bool, ScribbleError> {
        use std::process::Command;

        let output = Command::new("cargo")
        .arg("check")
        .current_dir(&self.project_root)
        .output()?;

        Ok(output.status.success())
    }

    pub fn save_changes(&self) -> Result<(), ScribbleError> {
        // Commit changes and remove backup
        if self.backup_path.exists() {
            fs::remove_dir_all(&self.backup_path)?;
        }
        Ok(())
    }

    pub fn revert_changes(&mut self) -> Result<(), ScribbleError> {
        // Restore from backup if exists
        if self.backup_path.exists() {
            for entry in fs::read_dir(&self.backup_path)? {
                let entry = entry?;
                let path = entry.path();
                let relative_path = path.strip_prefix(&self.backup_path)?;
                let target_path = self.project_root.join(relative_path);

                if path.is_file() {
                    fs::copy(path, target_path)?;
                }
            }
            fs::remove_dir_all(&self.backup_path)?;
        }
        self.edit_history.clear();
        Ok(())
    }

    fn create_backup(&self, file_path: &Path) -> Result<(), ScribbleError> {
        let relative_path = file_path.strip_prefix(&self.project_root)?;
        let backup_file = self.backup_path.join(relative_path);

        if let Some(parent) = backup_file.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(file_path, backup_file)?;
        Ok(())
    }
}

impl Drop for CargoEditor {
    fn drop(&mut self) {
        // Attempt to clean up backup directory
        if self.backup_path.exists() {
            let _ = fs::remove_dir_all(&self.backup_path);
        }
    }
}
