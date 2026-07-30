use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use error::{CoreError, Result};

pub fn extract_zip_streaming(
    archive_path: &str,
    target_dir: &str,
    callback: impl Fn(&str, u64),
) -> Result<Vec<String>> {
    let file = fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut extracted = Vec::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        let size = entry.size();

        callback(&name, size);

        let target_path = Path::new(target_dir).join(&name);

        if entry.is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut output = fs::File::create(&target_path)?;
            let mut buffer = [0u8; 65536];
            loop {
                let n = entry.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                output.write_all(&buffer[..n])?;
            }
        }
        extracted.push(name);
    }
    Ok(extracted)
}

pub fn extract_tar_gz_streaming(
    archive_path: &str,
    target_dir: &str,
    callback: impl Fn(&str, u64),
) -> Result<Vec<String>> {
    let file = fs::File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    let mut extracted = Vec::new();
    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?.to_string_lossy().to_string();
        let size = entry.header().size().unwrap_or(0);

        callback(&path, size);

        let target_path = Path::new(target_dir).join(&path);

        if entry.header().entry_type() == tar::EntryType::Directory {
            fs::create_dir_all(&target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            entry.unpack_in(target_dir)?;
        }
        extracted.push(path);
    }
    Ok(extracted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use zip::write::SimpleFileOptions;

    fn create_test_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let buf = Vec::new();
        let mut zw = zip::ZipWriter::new(Cursor::new(buf));
        let opts =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for (name, data) in entries {
            if name.ends_with('/') {
                zw.add_directory(*name, opts).unwrap();
            } else {
                zw.start_file(name, opts).unwrap();
                zw.write_all(data).unwrap();
            }
        }
        zw.finish().unwrap().into_inner()
    }

    #[test]
    fn test_streaming_extract() {
        let data = create_test_zip(&[("a.txt", b"aaa"), ("dir/b.txt", b"bbb")]);
        let temp = std::env::temp_dir().join("streaming_test");
        fs::create_dir_all(&temp).unwrap();
        let arc = temp.join("test.zip");
        fs::write(&arc, &data).unwrap();

        let target = temp.join("out");
        let mut progress = Vec::new();
        let files = extract_zip_streaming(
            arc.to_str().unwrap(),
            target.to_str().unwrap(),
            |name, size| progress.push((name.to_string(), size)),
        )
        .unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(progress.len(), 2);
        fs::remove_dir_all(&temp).unwrap();
    }
}
