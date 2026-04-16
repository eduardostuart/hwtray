//! Filesystem helpers shared across the crate.

use std::path::Path;

/// Write `contents` to `path` atomically using a sibling tempfile + rename.
///
/// On POSIX, `rename` within the same directory is atomic, so readers either
/// see the previous file or the new one — never a truncated in-progress write.
/// Uses async I/O so the caller's runtime is not blocked on disk latency.
pub async fn atomic_write(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let tmp_path = tmp_path_for(path);
    tokio::fs::write(&tmp_path, contents).await?;
    tokio::fs::rename(&tmp_path, path).await?;
    Ok(())
}

fn tmp_path_for(path: &Path) -> std::path::PathBuf {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("tmp");
    let mut tmp = path.to_path_buf();
    tmp.set_file_name(format!("{file_name}.tmp"));
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn writes_contents_to_target_path() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");

        atomic_write(&path, b"hello").await.unwrap();

        assert_eq!(tokio::fs::read(&path).await.unwrap(), b"hello");
    }

    #[tokio::test]
    async fn removes_tmp_file_on_success() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");

        atomic_write(&path, b"x").await.unwrap();

        let leftover: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".tmp"))
            .collect();
        assert!(leftover.is_empty(), "tmp files left: {leftover:?}");
    }

    #[tokio::test]
    async fn creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("dirs").join("out.json");

        atomic_write(&path, b"x").await.unwrap();

        assert!(path.exists());
    }

    #[tokio::test]
    async fn overwrites_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");

        atomic_write(&path, b"first").await.unwrap();
        atomic_write(&path, b"second").await.unwrap();

        assert_eq!(tokio::fs::read(&path).await.unwrap(), b"second");
    }

    #[test]
    fn tmp_path_appends_tmp_extension() {
        let tmp = tmp_path_for(Path::new("/tmp/foo.json"));
        assert_eq!(tmp, Path::new("/tmp/foo.json.tmp"));
    }

    #[test]
    fn tmp_path_handles_no_extension() {
        let tmp = tmp_path_for(Path::new("/tmp/foo"));
        assert_eq!(tmp, Path::new("/tmp/foo.tmp"));
    }
}
