use crate::clock::Clock;
use crate::models::project::Project;
use crate::models::session::Session;
use crate::repository::Repository;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Local};

pub struct SessionService {
    repository: Box<dyn Repository>,
    clock: Box<dyn Clock>,
}

pub struct StatusInfo {
    pub project_name: String,
    pub current_session: Option<Session>,
    pub last_session: Option<Session>,
    pub total_sessions: usize,
    pub current_time: DateTime<Local>,
}

impl SessionService {
    pub fn new(repository: Box<dyn Repository>, clock: Box<dyn Clock>) -> Self {
        Self { repository, clock }
    }

    pub fn init_project(&self, name: String) -> Result<()> {
        // Check if project already exists
        if self.repository.load().is_ok() {
            return Err(anyhow!("Project already initialized in this directory. Use a different directory or delete .clockme folder."));
        }

        let project = Project::new(name);
        self.repository.save(&project)?;
        Ok(())
    }

    pub fn start_session(&self) -> Result<Project> {
        let mut project = self
            .repository
            .load()
            .map_err(|_| anyhow!("No project found. Run 'clock-me init' first."))?;

        if project.current_session.is_some() {
            return Err(anyhow!("Already clocked in. Use 'clock-me out' first."));
        }

        let now = self.clock.now();
        project.start_session(now);
        self.repository.save(&project)?;

        Ok(project)
    }

    pub fn end_session(&self) -> Result<(Project, Duration)> {
        let mut project = self
            .repository
            .load()
            .map_err(|_| anyhow!("No project found. Run 'clock-me init' first."))?;

        if project.current_session.is_none() {
            return Err(anyhow!("Not clocked in. Use 'clock-me now' first."));
        }

        let now = self.clock.now();
        let duration = project.end_session(now)?;
        self.repository.save(&project)?;

        Ok((project, duration))
    }

    pub fn get_status(&self) -> Result<StatusInfo> {
        let project = self
            .repository
            .load()
            .map_err(|_| anyhow!("No project found. Run 'clock-me init' first."))?;

        let current_time = self.clock.now();
        let last_session = project.sessions.last().cloned();

        Ok(StatusInfo {
            project_name: project.name.clone(),
            current_session: project.current_session.clone(),
            last_session,
            total_sessions: project.sessions.len(),
            current_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::MockClock;
    use crate::repository::MockRepository;
    use chrono::TimeZone;

    #[test]
    fn test_init_project() {
        let mut mock_repo = MockRepository::new();
        mock_repo
            .expect_load()
            .returning(|| Err(anyhow!("Not found")));
        mock_repo.expect_save().returning(|_| Ok(()));

        let mock_clock = MockClock::new();
        let service = SessionService::new(Box::new(mock_repo), Box::new(mock_clock));

        assert!(service.init_project("test-project".to_string()).is_ok());
    }

    #[test]
    fn test_start_session_without_init() {
        let mut mock_repo = MockRepository::new();
        mock_repo
            .expect_load()
            .returning(|| Err(anyhow!("Not found")));

        let mock_clock = MockClock::new();
        let service = SessionService::new(Box::new(mock_repo), Box::new(mock_clock));

        assert!(service.start_session().is_err());
    }
}
