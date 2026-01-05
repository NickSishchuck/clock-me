use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

impl Break {
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
    fn test_new_break() {
        let start = Local.with_ymd_and_hms(2025, 10, 13, 12, 0, 0).unwrap();
        let break_period = Break::new(start);

        assert_eq!(break_period.start, start);
        assert!(break_period.end.is_none());
        assert!(break_period.is_active());
        assert!(break_period.duration().is_none());
    }

    #[test]
    fn test_finish_break() {
        let start = Local.with_ymd_and_hms(2025, 10, 13, 12, 0, 0).unwrap();
        let end = Local.with_ymd_and_hms(2025, 10, 13, 12, 15, 0).unwrap();

        let mut break_period = Break::new(start);
        break_period.finish(end);

        assert!(!break_period.is_active());
        assert_eq!(break_period.duration().unwrap().num_minutes(), 15);
    }
}
