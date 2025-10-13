use chrono::{DateTime, Local};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Local>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Local> {
        Local::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clock() {
        let clock = SystemClock;
        let now = clock.now();

        // Just verify it returns a valid datetime
        assert!(now.timestamp() > 0);
    }
}
