use error::CoreError;

pub trait ResultExt<T> {
    fn context(self, msg: &str) -> Result<T, CoreError>;
}

impl<T, E: std::fmt::Display> ResultExt<T> for Result<T, E> {
    fn context(self, msg: &str) -> Result<T, CoreError> {
        self.map_err(|e| CoreError::InvalidData(format!("{}: {}", msg, e)))
    }
}

pub fn chain_errors<T>(result: Result<T, CoreError>, context: &str) -> Result<T, CoreError> {
    result.map_err(|e| CoreError::InvalidData(format!("{}: {}", context, e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::CoreError;

    #[test]
    fn test_context() {
        let result: Result<i32, &str> = Err("something went wrong");
        let err = result.context("parsing config").unwrap_err();
        assert!(format!("{}", err).contains("parsing config"));
    }
}
