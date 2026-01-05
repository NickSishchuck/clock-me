use crate::models::r#break::Break;
use crate::models::session::Session;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub current_session: Option<Session>,
    #[serde(default)]
    pub current_break: Option<Break>,
    pub sessions: Vec<Session>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_session: None,
            current_break: None,
            sessions: Vec::new(),
        }
    }

    pub fn start_session(&mut self, start_time: DateTime<Local>) {
        self.current_session = Some(Session::new(start_time));
        self.current_break = None;
    }

    pub fn end_session(&mut self, end_time: DateTime<Local>) -> Result<Duration> {
        // If on break, end the break first
        if self.current_break.is_some() {
            self.end_break(end_time)?;
        }

        let mut session = self
            .current_session
            .take()
            .ok_or_else(|| anyhow!("No active session"))?;

        session.finish(end_time);
        let work_time = session
            .work_time()
            .ok_or_else(|| anyhow!("Session has no duration"))?;

        self.sessions.push(session);
        Ok(work_time)
    }

    pub fn start_break(&mut self, start_time: DateTime<Local>) -> Result<()> {
        if self.current_session.is_none() {
            return Err(anyhow!("Not clocked in. Use 'clock-me now' first."));
        }

        if self.current_break.is_some() {
            return Err(anyhow!(
                "Already on break. Use 'clock-me now' to continue working."
            ));
        }

        self.current_break = Some(Break::new(start_time));
        Ok(())
    }

    pub fn end_break(&mut self, end_time: DateTime<Local>) -> Result<Duration> {
        let mut break_period = self
            .current_break
            .take()
            .ok_or_else(|| anyhow!("Not on break"))?;

        break_period.finish(end_time);
        let duration = break_period
            .duration()
            .ok_or_else(|| anyhow!("Break has no duration"))?;

        // Add the completed break to the current session
        if let Some(ref mut session) = self.current_session {
            session.add_break(break_period);
        }

        Ok(duration)
    }

    pub fn is_on_break(&self) -> bool {
        self.current_break.is_some()
    }

    pub fn total_work_time(&self) -> Duration {
        self.sessions
            .iter()
            .filter_map(|s| s.work_time())
            .fold(Duration::zero(), |acc, d| acc + d)
    }

    pub fn total_break_time(&self) -> Duration {
        self.sessions
            .iter()
            .map(|s| s.total_break_time())
            .fold(Duration::zero(), |acc, d| acc + d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_new_project() {
        let project = Project::new("test".to_string());
        assert_eq!(project.name, "test");
        assert!(project.current_session.is_none());
        assert!(project.current_break.is_none());
        assert_eq!(project.sessions.len(), 0);
    }

    #[test]
    fn test_start_and_end_session() {
        let mut project = Project::new("test".to_string());
        let start = Local.with_ymd_and_hms(2025, 10, 13, 9, 0, 0).unwrap();
        let end = Local.with_ymd_and_hms(2025, 10, 13, 17, 0, 0).unwrap();

        project.start_session(start);
        assert!(project.current_session.is_some());

        let duration = project.end_session(end).unwrap();
        assert!(project.current_session.is_none());
        assert_eq!(project.sessions.len(), 1);
        assert_eq!(duration.num_hours(), 8);
    }

    #[test]
    fn test_break_management() {
        let mut project = Project::new("test".to_string());
        let start = Local.with_ymd_and_hms(2025, 10, 13, 9, 0, 0).unwrap();
        let break_start = Local.with_ymd_and_hms(2025, 10, 13, 12, 0, 0).unwrap();
        let break_end = Local.with_ymd_and_hms(2025, 10, 13, 12, 30, 0).unwrap();
        let end = Local.with_ymd_and_hms(2025, 10, 13, 17, 0, 0).unwrap();

        project.start_session(start);
        assert!(project.start_break(break_start).is_ok());
        assert!(project.is_on_break());

        let break_duration = project.end_break(break_end).unwrap();
        assert_eq!(break_duration.num_minutes(), 30);
        assert!(!project.is_on_break());

        let work_duration = project.end_session(end).unwrap();
        // 8 hours - 30 minutes = 7.5 hours
        assert_eq!(work_duration.num_minutes(), 8 * 60 - 30);
    }
}
