use error::Result;

pub fn verify_file_hash(filepath: &str, expected_hash: &str) -> Result<bool> {
    let bytes = std::fs::read(filepath)?;
    let computed = hashing::sha256(&bytes);
    Ok(computed.eq_ignore_ascii_case(expected_hash))
}

pub fn format_download_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec >= 1_000_000.0 {
        format!("{:.2} MB/s", bytes_per_sec / 1_000_000.0)
    } else if bytes_per_sec >= 1_000.0 {
        format!("{:.2} KB/s", bytes_per_sec / 1_000.0)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

pub fn estimate_time_remaining(bytes_remaining: u64, speed_bytes_per_sec: f64) -> String {
    if speed_bytes_per_sec <= 0.0 {
        return "unknown".to_string();
    }
    let secs = bytes_remaining as f64 / speed_bytes_per_sec;
    if secs >= 3600.0 {
        format!("{:.1}h", secs / 3600.0)
    } else if secs >= 60.0 {
        format!("{:.0}m {:02.0}s", secs / 60.0, secs % 60.0)
    } else {
        format!("{:.0}s", secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_speed() {
        assert_eq!(format_download_speed(500.0), "500 B/s");
        assert_eq!(format_download_speed(1500.0), "1.50 KB/s");
        assert_eq!(format_download_speed(2_500_000.0), "2.50 MB/s");
    }

    #[test]
    fn test_estimate_time() {
        assert_eq!(estimate_time_remaining(0, 1000.0), "0s");
        assert_eq!(estimate_time_remaining(5000, 1000.0), "5s");
        assert_eq!(estimate_time_remaining(120_000, 1000.0), "2m 00s");
    }
}
