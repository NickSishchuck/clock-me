use crate::models::session::Session;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub current_session: Option<Session>,
    pub sessions: Vec<Session>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_session: None,
            sessions: Vec::new(),
        }
    }

    pub fn start_session(&mut self, start_time: DateTime<Local>) {
        self.current_session = Some(Session::new(start_time));
    }

    pub fn end_session(&mut self, end_time: DateTime<Local>) -> Result<Duration> {
        let mut session = self
            .current_session
            .take()
            .ok_or_else(|| anyhow!("No active session"))?;

        session.finish(end_time);
        let duration = session
            .duration()
            .ok_or_else(|| anyhow!("Session has no duration"))?;

        self.sessions.push(session);
        Ok(duration)
    }

    pub fn total_time_tracked(&self) -> Duration {
        self.sessions
            .iter()
            .filter_map(|s| s.duration())
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
}
