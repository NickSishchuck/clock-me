use crate::models::r#break::Break;
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    #[serde(default)]
    pub breaks: Vec<Break>,
}

impl Session {
    pub fn new(start: DateTime<Local>) -> Self {
        Self {
            start,
            end: None,
            breaks: Vec::new(),
        }
    }

    pub fn finish(&mut self, end: DateTime<Local>) {
        self.end = Some(end);
    }

    pub fn duration(&self) -> Option<Duration> {
        self.end.map(|end| end.signed_duration_since(self.start))
    }

    pub fn is_active(&self) -> bool {
        self.end.is_none()
    }

    pub fn total_break_time(&self) -> Duration {
        self.breaks
            .iter()
            .filter_map(|b| b.duration())
            .fold(Duration::zero(), |acc, d| acc + d)
    }

    pub fn work_time(&self) -> Option<Duration> {
        self.duration().map(|total| total - self.total_break_time())
    }

    pub fn add_break(&mut self, break_period: Break) {
        self.breaks.push(break_period);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_new_session() {
        let start = Local.with_ymd_and_hms(2025, 10, 13, 9, 0, 0).unwrap();
        let session = Session::new(start);

        assert_eq!(session.start, start);
        assert!(session.end.is_none());
        assert!(session.is_active());
        assert!(session.duration().is_none());
        assert_eq!(session.breaks.len(), 0);
    }

    #[test]
    fn test_finish_session() {
        let start = Local.with_ymd_and_hms(2025, 10, 13, 9, 0, 0).unwrap();
        let end = Local.with_ymd_and_hms(2025, 10, 13, 17, 0, 0).unwrap();

        let mut session = Session::new(start);
        session.finish(end);

        assert!(!session.is_active());
        assert_eq!(session.duration().unwrap().num_hours(), 8);
    }

    #[test]
    fn test_session_with_breaks() {
        let start = Local.with_ymd_and_hms(2025, 10, 13, 9, 0, 0).unwrap();
        let end = Local.with_ymd_and_hms(2025, 10, 13, 17, 0, 0).unwrap();

        let mut session = Session::new(start);

        // Add a 30-minute break
        let break_start = Local.with_ymd_and_hms(2025, 10, 13, 12, 0, 0).unwrap();
        let break_end = Local.with_ymd_and_hms(2025, 10, 13, 12, 30, 0).unwrap();
        let mut break_period = Break::new(break_start);
        break_period.finish(break_end);
        session.add_break(break_period);

        session.finish(end);

        assert_eq!(session.duration().unwrap().num_hours(), 8);
        assert_eq!(session.total_break_time().num_minutes(), 30);
        assert_eq!(session.work_time().unwrap().num_minutes(), 8 * 60 - 30);
    }
}
