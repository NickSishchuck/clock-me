use crate::models::project::Project;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::fs;

#[cfg(test)]
use mockall::automock;

/// Repository trait for persistence abstraction
#[cfg_attr(test, automock)]
pub trait Repository: Send + Sync {
    fn load(&self) -> Result<Project>;
    fn save(&self, project: &Project) -> Result<()>;
}

/// File-based repository implementation
pub struct FileRepository {
    data_dir: PathBuf,
    data_file: PathBuf,
}

impl FileRepository {
    pub fn new() -> Self {
        let data_dir = PathBuf::from(".clockme");
        let data_file = data_dir.join("data.json");
        
        Self {
            data_dir,
            data_file,
        }
    }

    fn ensure_directory_exists(&self) -> Result<()> {
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir)
                .context("Failed to create .clockme directory")?;
        }
        Ok(())
    }
}

impl Repository for FileRepository {
    fn load(&self) -> Result<Project> {
        let content = fs::read_to_string(&self.data_file)
            .context("Failed to read project data. Has the project been initialized?")?;
        
        let project: Project = serde_json::from_str(&content)
            .context("Failed to parse project data")?;
        
        Ok(project)
    }

    fn save(&self, project: &Project) -> Result<()> {
        self.ensure_directory_exists()?;
        
        let json = serde_json::to_string_pretty(project)
            .context("Failed to serialize project data")?;
        
        fs::write(&self.data_file, json)
            .context("Failed to write project data")?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::project::Project;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().join(".clockme");
        let data_file = data_dir.join("data.json");
        
        let repo = FileRepository {
            data_dir,
            data_file,
        };

        let project = Project::new("test-project".to_string());
        
        assert!(repo.save(&project).is_ok());
        
        let loaded = repo.load().unwrap();
        assert_eq!(loaded.name, "test-project");
    }
}
