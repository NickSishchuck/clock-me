use crate::models::project::Project;
use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(test)]
use mockall::automock;

/// Repository trait for persistence abstraction
#[cfg_attr(test, automock)]
pub trait Repository: Send + Sync {
    fn load(&self) -> Result<Project>;
    fn save(&self, project: &Project) -> Result<()>;
}

pub struct FileRepository {
    data_dir: PathBuf,
    data_file: PathBuf,
}

impl FileRepository {
    pub fn new() -> Self {
        // Try to find .clockme by walking up the directory tree
        let (data_dir, data_file) = Self::find_clockme_dir().unwrap_or_else(|| {
            // If not found, use current directory (for init command)
            let data_dir = PathBuf::from(".clockme");
            let data_file = data_dir.join("data.json");
            (data_dir, data_file)
        });

        Self {
            data_dir,
            data_file,
        }
    }

    fn find_clockme_dir() -> Option<(PathBuf, PathBuf)> {
        let current_dir = env::current_dir().ok()?;
        let mut path = current_dir.as_path();

        loop {
            let clockme_dir = path.join(".clockme");
            if clockme_dir.exists() && clockme_dir.is_dir() {
                let data_file = clockme_dir.join("data.json");
                return Some((clockme_dir, data_file));
            }
            path = path.parent()?;
        }
    }

    pub fn get_project_root(&self) -> Option<PathBuf> {
        self.data_dir.parent().map(|p| p.to_path_buf())
    }

    fn ensure_directory_exists(&self) -> Result<()> {
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir).context("Failed to create .clockme directory")?;
        }
        Ok(())
    }
}

impl Repository for FileRepository {
    fn load(&self) -> Result<Project> {
        let content = fs::read_to_string(&self.data_file)
            .context("Failed to read project data. Has the project been initialized?")?;

        let project: Project =
            serde_json::from_str(&content).context("Failed to parse project data")?;

        Ok(project)
    }

    fn save(&self, project: &Project) -> Result<()> {
        self.ensure_directory_exists()?;

        let json =
            serde_json::to_string_pretty(project).context("Failed to serialize project data")?;

        fs::write(&self.data_file, json).context("Failed to write project data")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::project::Project;
    use std::fs;
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

    #[test]
    fn test_find_clockme_dir_in_subdirectory() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        // Create .clockme in root
        let clockme_dir = project_root.join(".clockme");
        fs::create_dir(&clockme_dir).unwrap();

        // Create a subdirectory
        let subdir = project_root.join("src").join("models");
        fs::create_dir_all(&subdir).unwrap();

        // Change to subdirectory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&subdir).unwrap();

        // Should find .clockme by walking up
        let result = FileRepository::find_clockme_dir();

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();

        assert!(result.is_some());
        let (found_dir, _) = result.unwrap();
        assert_eq!(found_dir, clockme_dir);
    }
}
