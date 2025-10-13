use anyhow::{anyhow, Result};
use regex::Regex;

pub struct ProjectValidator;

impl ProjectValidator {
    /// Validates that project name is alphanumeric with hyphens/underscores
    ///
    /// Rules:
    /// - Must not be empty
    /// - Must be 50 characters or less
    /// - Can only contain letters, numbers, hyphens, and underscores
    /// - Must start with a letter or number
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Project name cannot be empty"));
        }

        if name.len() > 50 {
            return Err(anyhow!("Project name must be 50 characters or less"));
        }

        // Pattern: starts with alphanumeric, can contain alphanumeric, hyphens, underscores
        let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]*$").unwrap();

        if !re.is_match(name) {
            return Err(anyhow!(
                "Project name must start with a letter or number and can only contain letters, numbers, hyphens, and underscores"
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_names() {
        assert!(ProjectValidator::validate_name("my-project").is_ok());
        assert!(ProjectValidator::validate_name("project_123").is_ok());
        assert!(ProjectValidator::validate_name("MyProject").is_ok());
        assert!(ProjectValidator::validate_name("project1").is_ok());
        assert!(ProjectValidator::validate_name("a").is_ok());
        assert!(ProjectValidator::validate_name("Project-Name_123").is_ok());
    }

    #[test]
    fn test_invalid_project_names() {
        // Empty name
        assert!(ProjectValidator::validate_name("").is_err());

        // Spaces not allowed
        assert!(ProjectValidator::validate_name("my project").is_err());

        // Special characters not allowed
        assert!(ProjectValidator::validate_name("project@home").is_err());
        assert!(ProjectValidator::validate_name("project!").is_err());
        assert!(ProjectValidator::validate_name("project.name").is_err());

        // Cannot start with hyphen or underscore
        assert!(ProjectValidator::validate_name("-project").is_err());
        assert!(ProjectValidator::validate_name("_project").is_err());

        // Too long (>50 characters)
        let long_name = "a".repeat(51);
        assert!(ProjectValidator::validate_name(&long_name).is_err());
    }

    #[test]
    fn test_error_messages() {
        let result = ProjectValidator::validate_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));

        let result = ProjectValidator::validate_name("my project");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must start with a letter"));

        let long_name = "a".repeat(51);
        let result = ProjectValidator::validate_name(&long_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("50 characters"));
    }
}
