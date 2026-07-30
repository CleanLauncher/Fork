use std::fmt;

#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub current_bytes: usize,
    pub peak_bytes: usize,
    pub allocations: usize,
}

impl MemoryReport {
    pub fn new() -> Self {
        MemoryReport {
            current_bytes: crate::core::get_current_memory_usage(),
            peak_bytes: 0,
            allocations: 0,
        }
    }

    pub fn current_kb(&self) -> f64 {
        self.current_bytes as f64 / 1024.0
    }

    pub fn current_mb(&self) -> f64 {
        self.current_bytes as f64 / (1024.0 * 1024.0)
    }
}

impl fmt::Display for MemoryReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Memory: {:.2} KB ({:.2} MB) - {} bytes",
            self.current_kb(),
            self.current_mb(),
            self.current_bytes
        )
    }
}

pub fn print_memory_usage() -> String {
    let report = MemoryReport::new();
    report.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_report() {
        let report = MemoryReport::new();
        assert!(report.current_mb() >= 0.0);
        let display = format!("{}", report);
        assert!(display.contains("Memory:"));
    }
}
