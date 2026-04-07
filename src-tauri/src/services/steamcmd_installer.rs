//! SteamCMD auto-installation.

use crate::services::retry::{retry, RetryConfig};
use crate::services::steam_errors::SteamCmdError;
use std::path::{Path, PathBuf};

/// SteamCMD download URLs.
const STEAMCMD_DOWNLOAD_URL_LINUX: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";
const STEAMCMD_DOWNLOAD_URL_WINDOWS: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";

/// Installer configuration.
#[derive(Debug, Clone)]
pub struct InstallerConfig {
    /// Target directory for SteamCMD installation.
    pub install_dir: PathBuf,
    /// Retry configuration.
    pub retry_config: RetryConfig,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            install_dir: PathBuf::from(".steamcmd"),
            retry_config: RetryConfig::default(),
        }
    }
}

impl InstallerConfig {
    /// Set a custom install directory.
    pub fn with_install_dir(mut self, dir: PathBuf) -> Self {
        self.install_dir = dir;
        self
    }
}

/// Download and install SteamCMD to the specified directory.
pub fn install_steamcmd(config: InstallerConfig) -> Result<PathBuf, SteamCmdError> {
    let install_dir = &config.install_dir;

    // Create install directory if it doesn't exist
    if !install_dir.exists() {
        std::fs::create_dir_all(install_dir)?;
    }

    // Determine download URL based on platform
    let download_url = if cfg!(target_os = "windows") {
        STEAMCMD_DOWNLOAD_URL_WINDOWS
    } else {
        STEAMCMD_DOWNLOAD_URL_LINUX
    };

    // Download with retry
    let archive_data = retry(config.retry_config.clone(), || {
        let url = download_url.to_string();
        download_file_sync(&url)
    })
    .map_err(|e: SteamCmdError| e)?;

    // Extract archive
    extract_steamcmd(&archive_data, install_dir)?;

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        let steamcmd_path = install_dir.join("steamcmd");
        if steamcmd_path.exists() {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&steamcmd_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&steamcmd_path, perms)?;
        }
    }

    Ok(install_dir.join(if cfg!(target_os = "windows") {
        "steamcmd.exe"
    } else {
        "steamcmd"
    }))
}

/// Download a file from URL synchronously.
fn download_file_sync(url: &str) -> Result<Vec<u8>, SteamCmdError> {
    // Use blocking reqwest call via tokio runtime
    let response =
        reqwest::blocking::get(url).map_err(|e| SteamCmdError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(SteamCmdError::DownloadFailed(format!(
            "HTTP {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .map_err(|e| SteamCmdError::NetworkError(e.to_string()))?;

    Ok(bytes.to_vec())
}

/// Extract steamcmd archive to target directory.
fn extract_steamcmd(data: &[u8], install_dir: &Path) -> Result<(), SteamCmdError> {
    if data.len() < 2 {
        return Err(SteamCmdError::InstallFailed("Invalid archive".to_string()));
    }

    // Detect archive type by magic bytes
    // ZIP: PK (0x50, 0x4B)
    // GZIP: 1f 8b
    if data[0] == 0x50 && data[1] == 0x4B {
        // ZIP format (Windows)
        extract_zip(data, install_dir)
    } else if data[0] == 0x1F && data[1] == 0x8B {
        // GZIP format (Linux)
        extract_tar_gz(data, install_dir)
    } else {
        Err(SteamCmdError::InstallFailed(
            "Unknown archive format".to_string(),
        ))
    }
}

/// Extract ZIP archive (Windows).
fn extract_zip(data: &[u8], install_dir: &Path) -> Result<(), SteamCmdError> {
    use std::io::Cursor;
    let cursor = Cursor::new(data);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| SteamCmdError::InstallFailed(e.to_string()))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = install_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

/// Extract tar.gz archive (Linux).
fn extract_tar_gz(data: &[u8], install_dir: &Path) -> Result<(), SteamCmdError> {
    let decoder = flate2::read::GzDecoder::new(data);
    let mut archive = tar::Archive::new(decoder);

    archive
        .unpack(install_dir)
        .map_err(|e| SteamCmdError::InstallFailed(e.to_string()))?;

    Ok(())
}

/// Sanitize install path to prevent path traversal.
pub fn sanitize_install_path(path: &Path) -> Result<PathBuf, SteamCmdError> {
    let canonical = path
        .canonicalize()
        .map_err(|_| SteamCmdError::PathTraversal)?;

    // Ensure the path doesn't escape home directory or similar
    let components: Vec<_> = canonical.components().collect();
    for component in &components {
        if let std::path::Component::ParentDir = component {
            return Err(SteamCmdError::PathTraversal);
        }
    }

    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = InstallerConfig::default();
        assert_eq!(config.install_dir, PathBuf::from(".steamcmd"));
        assert_eq!(config.retry_config.max_attempts, 3);
    }

    #[test]
    fn test_with_install_dir() {
        let config = InstallerConfig::default().with_install_dir(PathBuf::from("/custom/path"));
        assert_eq!(config.install_dir, PathBuf::from("/custom/path"));
    }
}
