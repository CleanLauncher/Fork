use error::{CoreError, Result};
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug, Clone)]
pub struct ChecksumSet {
    pub sha256: Option<String>,
    pub sha512: Option<String>,
    pub md5: Option<String>,
    pub crc32: Option<u32>,
    pub size: Option<u64>,
}

impl ChecksumSet {
    pub fn new() -> Self {
        ChecksumSet {
            sha256: None,
            sha512: None,
            md5: None,
            crc32: None,
            size: None,
        }
    }

    pub fn compute_all(data: &[u8]) -> Self {
        let mut set = ChecksumSet::new();
        set.sha256 = Some(crate::core::sha256(data));
        set.sha512 = Some(crate::core::sha512(data));
        set.md5 = Some(crate::core::md5(data));
        set.size = Some(data.len() as u64);
        set
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        if let Some(ref expected) = self.sha256 {
            if crate::core::sha256(data) != *expected {
                return false;
            }
        }
        if let Some(ref expected) = self.sha512 {
            if crate::core::sha512(data) != *expected {
                return false;
            }
        }
        if let Some(ref expected) = self.md5 {
            if crate::core::md5(data) != *expected {
                return false;
            }
        }
        if let Some(expected_size) = self.size {
            if data.len() as u64 != expected_size {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.sha256.is_none()
            && self.sha512.is_none()
            && self.md5.is_none()
            && self.crc32.is_none()
            && self.size.is_none()
    }
}

pub fn verify_file_integrity(path: &str, expected: &ChecksumSet) -> Result<bool> {
    let data = std::fs::read(path).map_err(CoreError::Io)?;
    Ok(expected.verify(&data))
}

pub fn compute_file_checksums(path: &str) -> Result<ChecksumSet> {
    let data = std::fs::read(path).map_err(CoreError::Io)?;
    Ok(ChecksumSet::compute_all(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_set() {
        let set = ChecksumSet::compute_all(b"hello world");
        assert!(set.sha256.is_some());
        assert!(set.sha512.is_some());
        assert!(set.md5.is_some());
        assert_eq!(set.size, Some(11));
        assert!(set.verify(b"hello world"));
        assert!(!set.verify(b"hello world!"));
    }

    #[test]
    fn test_empty_checksum_set() {
        let set = ChecksumSet::new();
        assert!(set.is_empty());
        assert!(set.verify(b"anything"));
    }
}
