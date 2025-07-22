//! Workspace management for PiCode
//! 
//! Manages project workspaces, file operations, and Git integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;
use ignore::gitignore::GitignoreBuilder;

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    pub root_path: PathBuf,
    pub ignore_patterns: Vec<String>,
    pub file_associations: HashMap<String, String>,
    pub git_enabled: bool,
    pub auto_save: bool,
    pub backup_enabled: bool,
    pub metadata: HashMap<String, String>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        let mut ignore_patterns = vec![
            "target/".to_string(),
            "node_modules/".to_string(),
            ".git/".to_string(),
            "*.tmp".to_string(),
            "*.log".to_string(),
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
        ];
        
        let mut file_associations = HashMap::new();
        file_associations.insert("rs".to_string(), "rust".to_string());
        file_associations.insert("py".to_string(), "python".to_string());
        file_associations.insert("js".to_string(), "javascript".to_string());
        file_associations.insert("ts".to_string(), "typescript".to_string());
        file_associations.insert("md".to_string(), "markdown".to_string());
        file_associations.insert("json".to_string(), "json".to_string());
        file_associations.insert("yaml".to_string(), "yaml".to_string());
        file_associations.insert("yml".to_string(), "yaml".to_string());
        file_associations.insert("toml".to_string(), "toml".to_string());
        
        Self {
            name: "default".to_string(),
            root_path: PathBuf::from("."),
            ignore_patterns,
            file_associations,
            git_enabled: true,
            auto_save: false,
            backup_enabled: true,
            metadata: HashMap::new(),
        }
    }
}

/// Workspace representation and operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub config: WorkspaceConfig,
    pub files: Vec<WorkspaceFile>,
    pub git_status: Option<GitStatus>,
    pub last_scan: chrono::DateTime<chrono::Utc>,
}

/// File information within a workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub file_type: FileType,
    pub language: Option<String>,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub is_binary: bool,
    pub git_status: Option<GitFileStatus>,
}

/// File type classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    Source,
    Config,
    Documentation,
    Asset,
    Build,
    Test,
    Unknown,
}

/// Git repository status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub is_dirty: bool,
    pub staged_files: usize,
    pub modified_files: usize,
    pub untracked_files: usize,
    pub remote_ahead: usize,
    pub remote_behind: usize,
}

/// Git file status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GitFileStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
}

impl Workspace {
    pub fn new(config: WorkspaceConfig) -> Self {
        Self {
            config,
            files: Vec::new(),
            git_status: None,
            last_scan: chrono::Utc::now(),
        }
    }
    
    pub async fn scan(&mut self) -> Result<(), WorkspaceError> {
        self.scan_files().await?;
        if self.config.git_enabled {
            self.scan_git().await?;
        }
        self.last_scan = chrono::Utc::now();
        Ok(())
    }
    
    async fn scan_files(&mut self) -> Result<(), WorkspaceError> {
        let mut files = Vec::new();
        let ignore_patterns = GitignoreBuilder::new(&self.config.root_path)
            .build()
            .map_err(|e| WorkspaceError::FileScan(e.to_string()))?;
        
        for entry in WalkDir::new(&self.config.root_path)
            .into_iter()
            .filter_entry(|e| !self.should_ignore(e.path()))
        {
            let entry = entry.map_err(|e| WorkspaceError::FileScan(e.to_string()))?;
            
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                let relative_path = path
                    .strip_prefix(&self.config.root_path)
                    .unwrap_or(&path)
                    .to_path_buf();
                
                let metadata = entry.metadata().map_err(|e| WorkspaceError::FileScan(e.to_string()))?;
                let file_type = self.classify_file(&relative_path);
                let language = self.detect_language(&relative_path);
                let is_binary = self.is_binary_file(&path).await;
                
                let file = WorkspaceFile {
                    path: path.clone(),
                    relative_path,
                    file_type,
                    language,
                    size: metadata.len(),
                    modified: metadata
                        .modified()
                        .map_err(|e| WorkspaceError::FileScan(e.to_string()))?
                        .into(),
                    is_binary,
                    git_status: None,
                };
                
                files.push(file);
            }
        }
        
        self.files = files;
        Ok(())
    }
    
    async fn scan_git(&mut self) -> Result<(), WorkspaceError> {
        let repo = match git2::Repository::open(&self.config.root_path) {
            Ok(repo) => repo,
            Err(_) => {
                // Not a git repository
                self.git_status = None;
                return Ok(());
            }
        };
        
        // Get current branch
        let head = repo.head().map_err(|e| WorkspaceError::Git(e.to_string()))?;
        let branch = head
            .shorthand()
            .unwrap_or("HEAD")
            .to_string();
        
        // Get status
        let statuses = repo
            .statuses(None)
            .map_err(|e| WorkspaceError::Git(e.to_string()))?;
        
        let mut staged_files = 0;
        let mut modified_files = 0;
        let mut untracked_files = 0;
        let is_dirty = !statuses.is_empty();
        
        // Update file git status
        for status in statuses.iter() {
            let status_flags = status.status();
            let file_path = PathBuf::from(status.path().unwrap_or(""));
            
            let git_status = if status_flags.is_index_new() || status_flags.is_index_modified() {
                staged_files += 1;
                Some(GitFileStatus::Added)
            } else if status_flags.is_wt_modified() {
                modified_files += 1;
                Some(GitFileStatus::Modified)
            } else if status_flags.is_wt_new() {
                untracked_files += 1;
                Some(GitFileStatus::Untracked)
            } else if status_flags.is_wt_deleted() {
                modified_files += 1;
                Some(GitFileStatus::Deleted)
            } else {
                Some(GitFileStatus::Unmodified)
            };
            
            // Update corresponding file
            if let Some(file) = self.files.iter_mut().find(|f| f.relative_path == file_path) {
                file.git_status = git_status;
            }
        }
        
        // TODO: Calculate remote ahead/behind (requires network operation)
        let remote_ahead = 0;
        let remote_behind = 0;
        
        self.git_status = Some(GitStatus {
            branch,
            is_dirty,
            staged_files,
            modified_files,
            untracked_files,
            remote_ahead,
            remote_behind,
        });
        
        Ok(())
    }
    
    fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        self.config.ignore_patterns.iter().any(|pattern| {
            if pattern.ends_with('/') {
                // Directory pattern
                path.is_dir() && path_str.contains(pattern.trim_end_matches('/'))
            } else if pattern.contains('*') {
                // Glob pattern (simple implementation)
                let pattern_without_star = pattern.replace('*', "");
                path_str.contains(&pattern_without_star)
            } else {
                // Exact match
                path_str.contains(pattern)
            }
        })
    }
    
    fn classify_file(&self, path: &Path) -> FileType {
        let path_str = path.to_string_lossy().to_lowercase();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());
        
        // Check by file name
        if path_str.contains("test") || path_str.contains("spec") {
            return FileType::Test;
        }
        
        if path_str.contains("readme") || path_str.contains("doc") || path_str.contains("changelog") {
            return FileType::Documentation;
        }
        
        // Check by extension
        match extension.as_deref() {
            Some("rs") | Some("py") | Some("js") | Some("ts") | Some("java") | Some("c") | Some("cpp") | Some("h") => FileType::Source,
            Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("ini") | Some("cfg") => FileType::Config,
            Some("md") | Some("txt") | Some("rst") | Some("adoc") => FileType::Documentation,
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") | Some("ico") => FileType::Asset,
            Some("lock") | Some("sum") => FileType::Build,
            _ => FileType::Unknown,
        }
    }
    
    fn detect_language(&self, path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| self.config.file_associations.get(ext))
            .cloned()
    }
    
    async fn is_binary_file(&self, path: &Path) -> bool {
        // Simple binary detection: read first few bytes and check for null bytes
        match tokio::fs::read(path).await {
            Ok(bytes) => {
                let sample_size = std::cmp::min(bytes.len(), 512);
                bytes[..sample_size].contains(&0)
            }
            Err(_) => false,
        }
    }
    
    pub fn get_files_by_type(&self, file_type: FileType) -> Vec<&WorkspaceFile> {
        self.files.iter().filter(|f| f.file_type == file_type).collect()
    }
    
    pub fn get_files_by_language(&self, language: &str) -> Vec<&WorkspaceFile> {
        self.files
            .iter()
            .filter(|f| f.language.as_deref() == Some(language))
            .collect()
    }
    
    pub fn find_file(&self, relative_path: &Path) -> Option<&WorkspaceFile> {
        self.files.iter().find(|f| f.relative_path == relative_path)
    }
    
    pub fn get_modified_files(&self) -> Vec<&WorkspaceFile> {
        self.files
            .iter()
            .filter(|f| matches!(f.git_status, Some(GitFileStatus::Modified)))
            .collect()
    }
    
    pub fn get_untracked_files(&self) -> Vec<&WorkspaceFile> {
        self.files
            .iter()
            .filter(|f| matches!(f.git_status, Some(GitFileStatus::Untracked)))
            .collect()
    }
    
    pub fn total_files(&self) -> usize {
        self.files.len()
    }
    
    pub fn total_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }
}

/// Workspace-related errors
#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("File scanning error: {0}")]
    FileScan(String),
    
    #[error("Git error: {0}")]
    Git(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Workspace not found: {0}")]
    NotFound(String),
    
    #[error("Invalid workspace configuration: {0}")]
    InvalidConfig(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn workspace_config_default() {
        let config = WorkspaceConfig::default();
        assert_eq!(config.name, "default");
        assert!(!config.ignore_patterns.is_empty());
        assert!(!config.file_associations.is_empty());
        assert!(config.git_enabled);
    }

    #[test]
    fn file_type_classification() {
        let config = WorkspaceConfig::default();
        let workspace = Workspace::new(config);
        
        assert_eq!(workspace.classify_file(Path::new("main.rs")), FileType::Source);
        assert_eq!(workspace.classify_file(Path::new("config.json")), FileType::Config);
        assert_eq!(workspace.classify_file(Path::new("README.md")), FileType::Documentation);
        assert_eq!(workspace.classify_file(Path::new("test_main.rs")), FileType::Test);
        assert_eq!(workspace.classify_file(Path::new("logo.png")), FileType::Asset);
        assert_eq!(workspace.classify_file(Path::new("unknown.xyz")), FileType::Unknown);
    }

    #[test]
    fn language_detection() {
        let config = WorkspaceConfig::default();
        let workspace = Workspace::new(config);
        
        assert_eq!(workspace.detect_language(Path::new("main.rs")), Some("rust".to_string()));
        assert_eq!(workspace.detect_language(Path::new("script.py")), Some("python".to_string()));
        assert_eq!(workspace.detect_language(Path::new("app.js")), Some("javascript".to_string()));
        assert_eq!(workspace.detect_language(Path::new("unknown.xyz")), None);
    }

    #[test]
    fn ignore_patterns() {
        let config = WorkspaceConfig::default();
        let workspace = Workspace::new(config);
        
        assert!(workspace.should_ignore(Path::new("target/")));
        assert!(workspace.should_ignore(Path::new("node_modules/")));
        assert!(workspace.should_ignore(Path::new("file.log")));
        assert!(workspace.should_ignore(Path::new("temp.tmp")));
        assert!(!workspace.should_ignore(Path::new("src/main.rs")));
    }

    #[tokio::test]
    async fn workspace_creation_and_scan() {
        let temp_dir = tempdir().unwrap();
        let workspace_path = temp_dir.path().to_path_buf();
        
        // Create some test files
        tokio::fs::write(workspace_path.join("main.rs"), "fn main() {}").await.unwrap();
        tokio::fs::write(workspace_path.join("README.md"), "# Test Project").await.unwrap();
        tokio::fs::create_dir(workspace_path.join("src")).await.unwrap();
        tokio::fs::write(workspace_path.join("src").join("lib.rs"), "// Library").await.unwrap();
        
        let mut config = WorkspaceConfig::default();
        config.root_path = workspace_path;
        config.git_enabled = false; // Disable git for this test
        
        let mut workspace = Workspace::new(config);
        workspace.scan().await.unwrap();
        
        assert!(!workspace.files.is_empty());
        assert!(workspace.find_file(Path::new("main.rs")).is_some());
        assert!(workspace.find_file(Path::new("README.md")).is_some());
        assert!(workspace.find_file(Path::new("src/lib.rs")).is_some());
        
        let rust_files = workspace.get_files_by_language("rust");
        assert_eq!(rust_files.len(), 2);
        
        let doc_files = workspace.get_files_by_type(FileType::Documentation);
        assert_eq!(doc_files.len(), 1);
    }

    #[test]
    fn git_file_status_classification() {
        assert_eq!(GitFileStatus::Modified, GitFileStatus::Modified);
        assert_ne!(GitFileStatus::Modified, GitFileStatus::Untracked);
    }

    #[test]
    fn workspace_file_queries() {
        let config = WorkspaceConfig::default();
        let mut workspace = Workspace::new(config);
        
        // Add some test files
        workspace.files.push(WorkspaceFile {
            path: PathBuf::from("main.rs"),
            relative_path: PathBuf::from("main.rs"),
            file_type: FileType::Source,
            language: Some("rust".to_string()),
            size: 100,
            modified: chrono::Utc::now(),
            is_binary: false,
            git_status: Some(GitFileStatus::Modified),
        });
        
        workspace.files.push(WorkspaceFile {
            path: PathBuf::from("new.rs"),
            relative_path: PathBuf::from("new.rs"),
            file_type: FileType::Source,
            language: Some("rust".to_string()),
            size: 50,
            modified: chrono::Utc::now(),
            is_binary: false,
            git_status: Some(GitFileStatus::Untracked),
        });
        
        assert_eq!(workspace.total_files(), 2);
        assert_eq!(workspace.total_size(), 150);
        assert_eq!(workspace.get_modified_files().len(), 1);
        assert_eq!(workspace.get_untracked_files().len(), 1);
        assert_eq!(workspace.get_files_by_language("rust").len(), 2);
    }
}