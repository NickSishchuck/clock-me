use anyhow::{anyhow, Result};
use chrono::Duration;
use regex::Regex;

pub struct DurationParser;

impl DurationParser {
    /// Parse duration strings like "2h 30m", "1h", "45m", "2h30m"
    ///
    /// Supported formats:
    /// - "2h" - hours only
    /// - "30m" - minutes only
    /// - "2h 30m" - hours and minutes with space
    /// - "2h30m" - hours and minutes without space
    /// - "1.5h" - decimal hours
    ///
    /// Returns a chrono::Duration
    pub fn parse(input: &str) -> Result<Duration> {
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            return Err(anyhow!("Duration cannot be empty"));
        }

        // Try to match hours and/or minutes pattern
        // Pattern explanation:
        // (?:(\d+(?:\.\d+)?)h)? - Optional hours (with optional decimal)
        // \s* - Optional whitespace
        // (?:(\d+)m)? - Optional minutes (integer only)
        let re = Regex::new(r"^(?:(\d+(?:\.\d+)?)h)?\s*(?:(\d+)m)?$").unwrap();

        if let Some(caps) = re.captures(&input) {
            let hours = caps
                .get(1)
                .and_then(|m| m.as_str().parse::<f64>().ok())
                .unwrap_or(0.0);

            let minutes = caps
                .get(2)
                .and_then(|m| m.as_str().parse::<i64>().ok())
                .unwrap_or(0);

            // Check if we got at least one value
            if hours == 0.0 && minutes == 0 {
                return Err(anyhow!(
                    "Invalid duration format. Use format like '2h 30m', '1.5h', '1h', or '45m'"
                ));
            }

            // Convert hours to minutes and add
            let total_minutes = (hours * 60.0) as i64 + minutes;

            if total_minutes <= 0 {
                return Err(anyhow!("Duration must be positive"));
            }

            Ok(Duration::minutes(total_minutes))
        } else {
            Err(anyhow!(
                "Invalid duration format. Use format like '2h 30m', '1.5h', '1h', or '45m'"
            ))
        }
    }

    /// Format a duration for display
    pub fn format(duration: Duration) -> String {
        let total_minutes = duration.num_minutes();
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        if hours > 0 && minutes > 0 {
            format!("{}h {}m", hours, minutes)
        } else if hours > 0 {
            format!("{}h", hours)
        } else {
            format!("{}m", minutes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hours_only() {
        let duration = DurationParser::parse("2h").unwrap();
        assert_eq!(duration.num_minutes(), 120);

        let duration = DurationParser::parse("1h").unwrap();
        assert_eq!(duration.num_minutes(), 60);
    }

    #[test]
    fn test_parse_minutes_only() {
        let duration = DurationParser::parse("30m").unwrap();
        assert_eq!(duration.num_minutes(), 30);

        let duration = DurationParser::parse("45m").unwrap();
        assert_eq!(duration.num_minutes(), 45);
    }

    #[test]
    fn test_parse_hours_and_minutes() {
        let duration = DurationParser::parse("2h 30m").unwrap();
        assert_eq!(duration.num_minutes(), 150);

        let duration = DurationParser::parse("1h 15m").unwrap();
        assert_eq!(duration.num_minutes(), 75);

        // Without space
        let duration = DurationParser::parse("2h30m").unwrap();
        assert_eq!(duration.num_minutes(), 150);
    }

    #[test]
    fn test_parse_decimal_hours() {
        let duration = DurationParser::parse("1.5h").unwrap();
        assert_eq!(duration.num_minutes(), 90);

        let duration = DurationParser::parse("2.25h").unwrap();
        assert_eq!(duration.num_minutes(), 135);

        let duration = DurationParser::parse("0.5h").unwrap();
        assert_eq!(duration.num_minutes(), 30);
    }

    #[test]
    fn test_parse_case_insensitive() {
        let duration = DurationParser::parse("2H").unwrap();
        assert_eq!(duration.num_minutes(), 120);

        let duration = DurationParser::parse("30M").unwrap();
        assert_eq!(duration.num_minutes(), 30);

        let duration = DurationParser::parse("2H 30M").unwrap();
        assert_eq!(duration.num_minutes(), 150);
    }

    #[test]
    fn test_parse_with_whitespace() {
        let duration = DurationParser::parse("  2h 30m  ").unwrap();
        assert_eq!(duration.num_minutes(), 150);

        let duration = DurationParser::parse("1h  15m").unwrap();
        assert_eq!(duration.num_minutes(), 75);
    }

    #[test]
    fn test_parse_invalid_formats() {
        assert!(DurationParser::parse("").is_err());
        assert!(DurationParser::parse("abc").is_err());
        assert!(DurationParser::parse("2x").is_err());
        assert!(DurationParser::parse("2h30").is_err());
        assert!(DurationParser::parse("h30m").is_err());
        assert!(DurationParser::parse("2.5m").is_err()); // decimal minutes not supported
    }

    #[test]
    fn test_parse_zero_duration() {
        assert!(DurationParser::parse("0h").is_err());
        assert!(DurationParser::parse("0m").is_err());
        assert!(DurationParser::parse("0h 0m").is_err());
    }

    #[test]
    fn test_format() {
        let duration = Duration::minutes(150);
        assert_eq!(DurationParser::format(duration), "2h 30m");

        let duration = Duration::minutes(120);
        assert_eq!(DurationParser::format(duration), "2h");

        let duration = Duration::minutes(45);
        assert_eq!(DurationParser::format(duration), "45m");

        let duration = Duration::minutes(75);
        assert_eq!(DurationParser::format(duration), "1h 15m");
    }
}
