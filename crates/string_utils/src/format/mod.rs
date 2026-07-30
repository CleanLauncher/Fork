use std::fmt::Write;

use chrono::{DateTime, Utc};

pub fn format_timestamp(secs: u64) -> String {
    let dt = DateTime::from_timestamp(secs as i64, 0).unwrap_or_default();
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn format_duration_seconds(total_secs: u64) -> String {
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {:02}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{} {}", count, singular)
    } else {
        format!("{} {}", count, plural)
    }
}

pub fn indent(text: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn format_list(items: &[String], conjunction: &str) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        2 => format!("{} {} {}", items[0], conjunction, items[1]),
        _ => {
            let mut result = String::new();
            for (i, item) in items.iter().enumerate() {
                if i == items.len() - 1 {
                    write!(result, "{} {}", conjunction, item).ok();
                } else if i > 0 {
                    write!(result, ", {}", item).ok();
                } else {
                    write!(result, "{}", item).ok();
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp() {
        let s = format_timestamp(0);
        assert!(s.contains("1970"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration_seconds(3661), "1h 01m 01s");
        assert_eq!(format_duration_seconds(125), "2m 05s");
        assert_eq!(format_duration_seconds(45), "45s");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(1, "file", "files"), "1 file");
        assert_eq!(pluralize(3, "file", "files"), "3 files");
    }

    #[test]
    fn test_format_list() {
        assert_eq!(format_list(&[], "and"), "");
        assert_eq!(format_list(&["a".into()], "and"), "a");
        assert_eq!(format_list(&["a".into(), "b".into()], "and"), "a and b");
        assert_eq!(
            format_list(&["a".into(), "b".into(), "c".into()], "or"),
            "a, b, or c"
        );
    }

    #[test]
    fn test_indent() {
        assert_eq!(indent("hello\nworld", 2), "  hello\n  world");
    }
}
