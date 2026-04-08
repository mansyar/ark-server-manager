//! Backup service — ZIP creation and retention enforcement.

use chrono::{DateTime, Utc};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};
use zip::{write::FileOptions, ZipWriter};

/// Describes a completed or failed backup operation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackupResult {
    /// Path to the created ZIP file, or None if failed.
    pub zip_path: Option<PathBuf>,
    /// Human-readable message (success or error).
    pub message: String,
    /// Number of backups retained after this operation.
    pub backups_retained: u32,
}

/// Returns the default backup directory for a profile.
///
/// Defaults to `{steamcmd_install_dir}/backups` if `backup_dir` is None.
pub fn resolve_backup_dir(steamcmd_install_dir: &Option<PathBuf>, backup_dir: &Option<PathBuf>) -> PathBuf {
    backup_dir.clone().unwrap_or_else(|| {
        steamcmd_install_dir
            .as_ref()
            .map(|d| d.join("backups"))
            .unwrap_or_else(|| {
                dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("ArkServerManager")
                    .join("backups")
            })
    })
}

/// Returns the timestamp-based filename for a new backup.
pub fn backup_filename(profile_name: &str, suffix: &str, timestamp: &DateTime<Utc>) -> String {
    format!(
        "{}_{}_{}.zip",
        profile_name,
        suffix,
        timestamp.format("%Y%m%d_%H%M%S")
    )
}

/// Creates a ZIP backup of the ARK server installation directory.
///
/// - `source_dir` — root ARK server install directory (contains ShooterGame/Binaries/Win64).
/// - `backup_dir` — directory where the ZIP file will be written.
/// - `profile_name` — used to build the ZIP filename.
/// - `suffix` — used to build the ZIP filename.
/// - `retention_count` — maximum backups to retain; 0 means keep all.
///
/// On success returns `BackupResult` with `zip_path` set.
/// On failure returns `BackupResult` with `zip_path: None` and an error message.
pub fn create_backup(
    source_dir: &Path,
    steamcmd_install_dir: &Option<PathBuf>,
    backup_dir: &Option<PathBuf>,
    profile_name: &str,
    suffix: &str,
    retention_count: u32,
) -> BackupResult {
    let dest_dir = resolve_backup_dir(steamcmd_install_dir, backup_dir);

    // Ensure backup directory exists
    if let Err(e) = fs::create_dir_all(&dest_dir) {
        error!("Failed to create backup directory {:?}: {}", dest_dir, e);
        return BackupResult {
            zip_path: None,
            message: format!("Failed to create backup directory: {}", e),
            backups_retained: 0,
        };
    }

    let timestamp = Utc::now();
    let filename = backup_filename(profile_name, suffix, &timestamp);
    let zip_path = dest_dir.join(&filename);

    info!(
        "Creating backup: source={:?}, dest={:?}",
        source_dir, zip_path
    );

    // Estimate total size for progress tracking (optional, not strictly needed)
    let mut zip_writer = match File::create(&zip_path) {
        Ok(file) => ZipWriter::new(file),
        Err(e) => {
            error!("Failed to create ZIP file {:?}: {}", zip_path, e);
            return BackupResult {
                zip_path: None,
                message: format!("Failed to create ZIP file: {}", e),
                backups_retained: 0,
            };
        }
    };

    let options: zip::write::FileOptions<'_, zip::write::SimpleFileOptions> =
        zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(5));

    let files_added: u32 = 0;

    // Walk the source directory and add each file to the ZIP
    match walk_dir(source_dir, &mut zip_writer, source_dir, options) {
        Ok(_) => {
            if let Err(e) = zip_writer.finish() {
                error!("Failed to finalize ZIP file: {}", e);
                // Clean up partial file
                let _ = fs::remove_file(&zip_path);
                return BackupResult {
                    zip_path: None,
                    message: format!("Failed to finalize ZIP: {}", e),
                    backups_retained: 0,
                };
            }
            info!(
                "Backup completed: {} files archived to {:?}",
                files_added, zip_path
            );
        }
        Err(e) => {
            error!("Failed to write ZIP contents: {}", e);
            let _ = fs::remove_file(&zip_path);
            return BackupResult {
                zip_path: None,
                message: format!("Failed to write ZIP contents: {}", e),
                backups_retained: 0,
            };
        }
    }

    // Enforce retention count
    let backups_retained = if retention_count > 0 {
        enforce_retention(&dest_dir, profile_name, suffix, retention_count)
    } else {
        count_backups(&dest_dir, profile_name, suffix)
    };

    BackupResult {
        zip_path: Some(zip_path),
        message: format!("Backup created successfully ({} files)", files_added),
        backups_retained,
    }
}

/// Recursively walks `dir` and writes each file into `zip_writer`.
fn walk_dir<W: Write + io::Seek>(
    dir: &Path,
    zip_writer: &mut ZipWriter<W>,
    base_dir: &Path,
    options: zip::write::FileOptions<'_, zip::write::SimpleFileOptions>,
) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(&path, zip_writer, base_dir, options)?;
        } else {
            let relative_path = path.strip_prefix(base_dir).unwrap_or(&path);
            let name = relative_path.to_string_lossy().replace('\\', "/");

            debug!("Adding to ZIP: {}", name);
            zip_writer.start_file(&name, options)?;
            let mut file = File::open(&path)?;
            io::copy(&mut file, zip_writer)?;
        }
    }

    Ok(())
}

/// Counts how many backup ZIP files exist for a profile.
fn count_backups(backup_dir: &Path, profile_name: &str, suffix: &str) -> u32 {
    let prefix = format!("{}_{}", profile_name, suffix);
    fs::read_dir(backup_dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with(&prefix) && n.ends_with(".zip"))
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0) as u32
}

/// Deletes the oldest backup(s) to enforce the retention count.
fn enforce_retention(
    backup_dir: &Path,
    profile_name: &str,
    suffix: &str,
    retention_count: u32,
) -> u32 {
    let prefix = format!("{}_{}", profile_name, suffix);

    let mut entries: Vec<_> = fs::read_dir(backup_dir)
        .map(|e| e.flatten().collect::<Vec<_>>())
        .unwrap_or_default();

    // Filter to matching backup files
    entries.retain(|e| {
        e.path()
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with(&prefix) && n.ends_with(".zip"))
            .unwrap_or(false)
    });

    // Sort by modification time (oldest first)
    entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());

    let excess = entries.len() as i64 - retention_count as i64;

    if excess <= 0 {
        return entries.len() as u32;
    }

    let to_delete = entries.into_iter().take(excess as usize);

    for entry in to_delete {
        if let Err(e) = fs::remove_file(entry.path()) {
            warn!("Failed to delete old backup {:?}: {}", entry.path(), e);
        } else {
            info!(
                "Deleted old backup: {:?}",
                entry.path().file_name().unwrap_or_default()
            );
        }
    }

    count_backups(backup_dir, profile_name, suffix)
}

/// Lists all backup ZIP files for a profile, sorted newest first.
pub fn list_backups(
    steamcmd_install_dir: &Option<PathBuf>,
    backup_dir: &Option<PathBuf>,
    profile_name: &str,
    suffix: &str,
) -> Vec<PathBuf> {
    let dest_dir = resolve_backup_dir(steamcmd_install_dir, backup_dir);
    let prefix = format!("{}_{}", profile_name, suffix);

    let mut entries: Vec<_> = fs::read_dir(&dest_dir)
        .map(|e| {
            e.flatten()
                .filter(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with(&prefix) && n.ends_with(".zip"))
                        .unwrap_or(false)
                })
                .collect()
        })
        .unwrap_or_default();

    // Sort newest first
    entries.sort_by_key(|e| std::cmp::Reverse(e.metadata().and_then(|m| m.modified()).ok()));

    entries.into_iter().map(|e| e.path()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_dummy_server(root: &Path) {
        // Create a fake ShooterGame/Binaries/Win64/ShooterGameServer.exe structure
        let exe_dir = root.join("ShooterGame").join("Binaries").join("Win64");
        fs::create_dir_all(&exe_dir).unwrap();
        let exe_path = exe_dir.join("ShooterGameServer.exe");
        fs::write(&exe_path, b"dummy executable").unwrap();

        let saves_dir = root.join("ShooterGame").join("Saved").join("SavedArks");
        fs::create_dir_all(&saves_dir).unwrap();
        let save_file = saves_dir.join("TheIsland.arkprofile");
        fs::write(&save_file, b"save data").unwrap();
    }

    #[test]
    fn test_resolve_backup_dir_explicit() {
        let explicit = PathBuf::from("/custom/backups");
        let result = resolve_backup_dir(&None, &Some(explicit.clone()));
        assert_eq!(result, explicit);
    }

    #[test]
    fn test_resolve_backup_dir_default() {
        let install_dir = PathBuf::from("/ark/server");
        let result = resolve_backup_dir(&Some(install_dir.clone()), &None);
        assert_eq!(result, install_dir.join("backups"));
    }

    #[test]
    fn test_resolve_backup_dir_totally_fallback() {
        let result = resolve_backup_dir(&None, &None);
        let expected = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ArkServerManager")
            .join("backups");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_backup_filename_format() {
        let ts = DateTime::parse_from_rfc3339("2025-01-15T10:30:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let name = backup_filename("MyServer", "backup", &ts);
        assert_eq!(name, "MyServer_backup_20250115_103000.zip");
    }

    #[test]
    fn test_create_backup_and_retention() {
        let temp_source = TempDir::new().unwrap();
        let temp_backup = TempDir::new().unwrap();

        create_dummy_server(temp_source.path());

        let result = create_backup(
            temp_source.path(),
            &None,
            &Some(temp_backup.path().to_path_buf()),
            "TestServer",
            "backup",
            3, // retain max 3
        );

        assert!(result.zip_path.is_some(), "Backup should succeed: {}", result.message);
        assert!(result.message.contains("successfully"));

        // Create 5 more backups
        for i in 0..5 {
            let ts = Utc::now();
            let ts = ts + chrono::Duration::seconds(i as i64 * 10);
            let filename = backup_filename("TestServer", "backup", &ts);
            let zip_path = temp_backup.path().join(&filename);
            let file = File::create(&zip_path).unwrap();
            let mut zw = ZipWriter::new(file);
            zw.start_file("test.txt", zip::write::SimpleFileOptions::default()).unwrap();
            zw.write_all(b"data").unwrap();
            zw.finish().unwrap();
        }

        // Now enforce retention by calling create_backup again
        // (we do this in a simpler way by directly calling enforce_retention)
        let count = count_backups(temp_backup.path(), "TestServer", "backup");
        assert_eq!(count, 6); // 6 total

        let retained = enforce_retention(temp_backup.path(), "TestServer", "backup", 3);
        assert_eq!(retained, 3);
    }

    #[test]
    fn test_list_backups_sorted_newest_first() {
        let temp = TempDir::new().unwrap();

        // Create some dummy zip files with different timestamps
        for i in 0..4 {
            let ts = Utc::now() - chrono::Duration::hours(i as i64);
            let filename = backup_filename("Server", "backup", &ts);
            let path = temp.path().join(&filename);
            let file = File::create(&path).unwrap();
            let mut zw = ZipWriter::new(file);
            zw.start_file("a.txt", Default::default()).unwrap();
            zw.write_all(b"x").unwrap();
            zw.finish().unwrap();

            // Ensure different modification times
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let backups = list_backups(&None, &Some(temp.path().to_path_buf()), "Server", "backup");
        assert_eq!(backups.len(), 4);

        // Verify sorted newest first
        for i in 0..backups.len() - 1 {
            let t1 = fs::metadata(&backups[i])
                .and_then(|m| m.modified())
                .ok();
            let t2 = fs::metadata(&backups[i + 1])
                .and_then(|m| m.modified())
                .ok();
            assert!(t1 >= t2, "Backups should be sorted newest first");
        }
    }
}
