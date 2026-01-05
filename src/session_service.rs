use crate::clock::Clock;
use crate::models::project::Project;
use crate::models::session::Session;
use crate::repository::Repository;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Local, Timelike};

pub struct SessionService {
    repository: Box<dyn Repository>,
    clock: Box<dyn Clock>,
}

pub struct StatusInfo {
    pub project_name: String,
    pub current_session: Option<Session>,
    pub current_break_start: Option<DateTime<Local>>,
    pub last_session: Option<Session>,
    pub total_sessions: usize,
    pub current_time: DateTime<Local>,
    pub today_work_time: Duration,
    pub today_break_time: Duration,
    pub today_sessions: usize,
    pub today_breaks: usize,
    pub total_work_time: Duration,
    pub total_break_time: Duration,
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

        if project.is_on_break() {
            let now = self.clock.now();
            project.end_break(now)?;
            self.repository.save(&project)?;
            return Ok(project);
        }

        if project.current_session.is_some() {
            return Err(anyhow!("Already clocked in. Use 'clock-me stop' first."));
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

    pub fn start_break(&self) -> Result<(Project, Duration)> {
        let mut project = self
            .repository
            .load()
            .map_err(|_| anyhow!("No project found. Run 'clock-me init' first."))?;

        let now = self.clock.now();
        project.start_break(now)?;

        // Calculate session work time so far (before break)
        let session_work_time = if let Some(ref session) = project.current_session {
            let elapsed = now.signed_duration_since(session.start);
            let break_time = session.total_break_time();
            elapsed - break_time
        } else {
            Duration::zero()
        };

        self.repository.save(&project)?;

        Ok((project, session_work_time))
    }

    pub fn get_status(&self) -> Result<StatusInfo> {
        let project = self
            .repository
            .load()
            .map_err(|_| anyhow!("No project found. Run 'clock-me init' first."))?;

        let current_time = self.clock.now();
        let last_session = project.sessions.last().cloned();

        // Calculate today's stats
        let today_start = current_time
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let (today_work_time, today_break_time, today_sessions, today_breaks) =
            self.calculate_today_stats(&project, today_start, current_time);

        // Calculate total stats (including current session if active)
        let mut total_work_time = project.total_work_time();
        let mut total_break_time = project.total_break_time();

        if let Some(ref session) = project.current_session {
            let elapsed = current_time.signed_duration_since(session.start);
            let session_break_time = session.total_break_time();

            // Add current break time if on break
            let current_break_time = if let Some(ref break_period) = project.current_break {
                current_time.signed_duration_since(break_period.start)
            } else {
                Duration::zero()
            };

            total_work_time = total_work_time + elapsed - session_break_time - current_break_time;
            total_break_time = total_break_time + session_break_time + current_break_time;
        }

        Ok(StatusInfo {
            project_name: project.name.clone(),
            current_session: project.current_session.clone(),
            current_break_start: project.current_break.as_ref().map(|b| b.start),
            last_session,
            total_sessions: project.sessions.len(),
            current_time,
            today_work_time,
            today_break_time,
            today_sessions,
            today_breaks,
            total_work_time,
            total_break_time,
        })
    }

    fn calculate_today_stats(
        &self,
        project: &Project,
        today_start: DateTime<Local>,
        current_time: DateTime<Local>,
    ) -> (Duration, Duration, usize, usize) {
        let mut work_time = Duration::zero();
        let mut break_time = Duration::zero();
        let mut session_count = 0;
        let mut break_count = 0;

        // Process completed sessions from today
        for session in &project.sessions {
            if session.start >= today_start {
                session_count += 1;
                if let Some(wt) = session.work_time() {
                    work_time = work_time + wt;
                }
                break_time = break_time + session.total_break_time();
                break_count += session.breaks.len();
            }
        }

        // Add current session if it started today
        if let Some(ref session) = project.current_session {
            if session.start >= today_start {
                session_count += 1;

                let elapsed = current_time.signed_duration_since(session.start);
                let session_break_time = session.total_break_time();

                // Add current break time if on break
                let current_break_time = if let Some(ref break_period) = project.current_break {
                    current_time.signed_duration_since(break_period.start)
                } else {
                    Duration::zero()
                };

                work_time = work_time + elapsed - session_break_time - current_break_time;
                break_time = break_time + session_break_time + current_break_time;
                break_count += session.breaks.len();

                if project.current_break.is_some() {
                    break_count += 1;
                }
            }
        }

        (work_time, break_time, session_count, break_count)
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
