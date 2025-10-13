use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

impl Session {
    pub fn new(start: DateTime<Local>) -> Self {
        Self { start, end: None }
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
}
