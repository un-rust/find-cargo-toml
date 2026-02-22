//! Find `Cargo.toml` (or a custom manifest filename) by walking up the directory tree.
//!
//! Starts from a given path and yields every directory that contains the manifest file,
//! from nearest to the root. Useful for locating workspace or package roots in Rust projects.

use std::path::{Path, PathBuf};

/// Iterator that walks upward from a directory and yields the full path to the manifest file
/// whenever it exists. Yields paths from the directory nearest to the start path toward the root.
pub struct FindIter {
    current: Option<PathBuf>,
    file_name: String,
}

impl Iterator for FindIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        while let Some(ref dir) = self.current {
            let candidate = dir.join(&self.file_name);
            if candidate.is_file() {
                let result = candidate;
                self.current = dir.parent().map(PathBuf::from);
                return Some(result);
            }
            self.current = dir.parent().map(PathBuf::from);
        }
        None
    }
}

/// Finds manifest files by walking up from `input`. Defaults to `Cargo.toml`.
///
/// # Arguments
///
/// * **`input`** – Where to start. Can be a directory or a file path; if a file,
///   its parent directory is used. Relative paths are resolved against `base`.
/// * **`base`** – Base path for resolving relative `input`. If `None`, the current
///   working directory is used.
/// * **`file_name`** – Name of the manifest file to look for. If `None`, `"Cargo.toml"` is used.
///
/// # Returns
///
/// A [`FindIter`] that yields the full path to each manifest file found, from the directory
/// closest to the start path upward toward the filesystem root.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use find_cargo_toml::find;
///
/// for path in find(".", None::<PathBuf>, None) {
///     println!("Found: {}", path.display());
/// }
/// ```
pub fn find<P, Q>(input: P, base: Option<Q>, file_name: Option<&str>) -> FindIter
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file_name = file_name.unwrap_or("Cargo.toml").to_string();
    let base: PathBuf = base
        .map(|b| b.as_ref().to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let start: PathBuf = if input.as_ref().is_absolute() {
        input.as_ref().to_path_buf()
    } else {
        base.join(input.as_ref())
    };
    let start_normalized = normalize_path(&start);
    let start_dir = if start_normalized.is_file() {
        start_normalized
            .parent()
            .map(PathBuf::from)
            .unwrap_or(start_normalized)
    } else {
        start_normalized
    };

    FindIter {
        current: Some(start_dir),
        file_name,
    }
}

/// Convenience wrapper for [`find`] that uses the current working directory as the base.
/// Equivalent to `find(input, None::<PathBuf>, file_name)`.
pub fn find_from_current_dir<P>(input: P, file_name: Option<&str>) -> FindIter
where
    P: AsRef<Path>,
{
    find(input, None::<PathBuf>, file_name)
}

/// Resolves `.` and `..` in `path` and returns a normalized [`PathBuf`].
fn normalize_path(path: &Path) -> PathBuf {
    path.components().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn find_yields_nothing_when_no_cargo_toml() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_empty");
        let _ = fs::create_dir_all(&tmp);
        let count = find(&tmp, None::<PathBuf>, None).count();
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(count, 0);
    }

    #[test]
    fn find_yields_path_when_cargo_toml_in_dir() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_with");
        let _ = fs::create_dir_all(&tmp);
        let manifest = tmp.join("Cargo.toml");
        let _ = fs::File::create(&manifest).and_then(|mut f| f.write_all(b"[package]"));
        let collected: Vec<_> = find(&tmp, None::<PathBuf>, None).collect();
        let _ = fs::remove_file(manifest);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("Cargo.toml"));
    }

    #[test]
    fn find_respects_custom_file_name() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_custom");
        let _ = fs::create_dir_all(&tmp);
        let custom = tmp.join("MyManifest.toml");
        let _ = fs::File::create(&custom).and_then(|mut f| f.write_all(b"[package]"));
        let collected: Vec<_> = find(&tmp, None::<PathBuf>, Some("MyManifest.toml")).collect();
        let _ = fs::remove_file(custom);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("MyManifest.toml"));
    }

    #[test]
    fn find_with_absolute_input() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_absolute");
        let _ = fs::create_dir_all(&tmp);
        let manifest = tmp.join("Cargo.toml");
        let _ = fs::File::create(&manifest).and_then(|mut f| f.write_all(b"[package]"));
        let abs = tmp.canonicalize().unwrap();
        let collected: Vec<_> = find(&abs, None::<PathBuf>, None).collect();
        let _ = fs::remove_file(manifest);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("Cargo.toml"));
    }

    #[test]
    fn find_when_input_is_file_uses_parent_dir() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_file_input");
        let _ = fs::create_dir_all(&tmp);
        let manifest = tmp.join("Cargo.toml");
        let _ = fs::File::create(&manifest).and_then(|mut f| f.write_all(b"[package]"));
        let some_file = tmp.join("foo.rs");
        let _ = fs::File::create(&some_file);
        let collected: Vec<_> = find(&some_file, None::<PathBuf>, None).collect();
        let _ = fs::remove_file(some_file);
        let _ = fs::remove_file(manifest);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("Cargo.toml"));
    }

    #[test]
    fn find_from_current_dir_delegates_to_find() {
        let count = find_from_current_dir(".", None).count();
        assert!(count >= 1, "project root has Cargo.toml");
    }

    #[test]
    fn find_with_explicit_base() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_base");
        let _ = fs::create_dir_all(&tmp);
        let manifest = tmp.join("Cargo.toml");
        let _ = fs::File::create(&manifest).and_then(|mut f| f.write_all(b"[package]"));
        let collected: Vec<_> = find(".", Some(&tmp), None).collect();
        let _ = fs::remove_file(manifest);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(collected.len(), 1);
        assert!(collected[0].ends_with("Cargo.toml"));
    }

    #[test]
    fn find_normalizes_path_with_dot_dot() {
        let tmp = std::env::temp_dir().join("find_cargo_toml_test_normalize");
        let sub = tmp.join("sub");
        let _ = fs::create_dir_all(&sub);
        let manifest = tmp.join("Cargo.toml");
        let _ = fs::File::create(&manifest).and_then(|mut f| f.write_all(b"[package]"));
        let input = sub.join("..");
        let collected: Vec<_> = find(&input, None::<PathBuf>, None).collect();
        let _ = fs::remove_file(manifest);
        let _ = fs::remove_dir_all(&tmp);
        assert!(
            collected
                .iter()
                .any(|p| p.parent().map(|d| d == tmp).unwrap_or(false)),
            "should find Cargo.toml in tmp when input is sub/.."
        );
    }
}
