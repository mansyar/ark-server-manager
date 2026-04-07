//! SteamCMD progress parsing and tracking.

use regex::Regex;

/// Progress information from SteamCMD output.
#[derive(Debug, Clone)]
pub struct SteamProgress {
    /// Download percentage (0-100).
    pub percentage: Option<f64>,
    /// Download speed in bytes per second.
    pub speed_bps: Option<u64>,
    /// Estimated time remaining in seconds.
    pub eta_seconds: Option<u64>,
    /// Raw line that was parsed.
    pub raw_line: String,
}

impl SteamProgress {
    /// Create a new progress struct.
    pub fn new(raw_line: String) -> Self {
        Self {
            percentage: None,
            speed_bps: None,
            eta_seconds: None,
            raw_line,
        }
    }
}

/// Parse a SteamCMD output line for progress information.
pub fn parse_progress_line(line: &str) -> Option<SteamProgress> {
    let line = line.trim();

    // Pattern for download progress: "Downloading... 50% (12345678 / 12345678 bytes)"
    // Also handles: "Update mediamanager (49/50)..."
    let percentage_re = Regex::new(r"(\d+(?:\.\d+)?)\s*%").ok()?;
    let bytes_re = Regex::new(r"\((\d+)\s*/\s*(\d+)\s+bytes\)").ok()?;
    let eta_re = Regex::new(r"ETA\s+(\d+:\d+:\d+)").ok()?;
    let speed_re = Regex::new(r"@\s*([\d.]+\s*[KMG]?B/s)").ok()?;

    let mut progress = SteamProgress::new(line.to_string());

    // Extract percentage
    if let Some(caps) = percentage_re.captures(line) {
        progress.percentage = caps.get(1).and_then(|m| m.as_str().parse().ok());
    }

    // Extract bytes if available
    if let Some(caps) = bytes_re.captures(line) {
        // We have download progress
        let downloaded: u64 = caps.get(1)?.as_str().parse().ok()?;
        let total: u64 = caps.get(2)?.as_str().parse().ok()?;
        if total > 0 {
            progress.percentage = Some((downloaded as f64 / total as f64) * 100.0);
        }
    }

    // Extract speed
    if let Some(caps) = speed_re.captures(line) {
        let speed_str = caps.get(1)?.as_str();
        progress.speed_bps = parse_speed(speed_str);
    }

    // Extract ETA
    if let Some(caps) = eta_re.captures(line) {
        let eta_str = caps.get(1)?.as_str();
        progress.eta_seconds = parse_eta(eta_str);
    }

    // Return None if no progress info found
    if progress.percentage.is_none()
        && progress.speed_bps.is_none()
        && progress.eta_seconds.is_none()
    {
        return None;
    }

    Some(progress)
}

/// Parse speed string like "1.5 MB/s" into bytes per second.
fn parse_speed(speed_str: &str) -> Option<u64> {
    let speed_str = speed_str.trim();
    let re = Regex::new(r"([\d.]+)\s*([KMG]?B)/s").ok()?;

    let caps = re.captures(speed_str)?;
    let value: f64 = caps.get(1)?.as_str().parse().ok()?;
    let unit = caps.get(2)?.as_str();

    let multiplier = match unit {
        "B" => 1,
        "KB" => 1024,
        "MB" => 1024 * 1024,
        "GB" => 1024 * 1024 * 1024,
        _ => return None,
    };

    Some((value * multiplier as f64) as u64)
}

/// Parse ETA string like "00:05:30" into seconds.
fn parse_eta(eta_str: &str) -> Option<u64> {
    let parts: Vec<&str> = eta_str.split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let hours: u64 = parts[0].parse().ok()?;
    let minutes: u64 = parts[1].parse().ok()?;
    let seconds: u64 = parts[2].parse().ok()?;

    Some(hours * 3600 + minutes * 60 + seconds)
}

/// Calculate download size estimate from progress.
pub fn estimate_total_size(downloaded: u64, percentage: f64) -> Option<u64> {
    if percentage > 0.0 && percentage <= 100.0 {
        Some((downloaded as f64 / percentage * 100.0) as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_percentage() {
        let line = "Downloading... 50% (12345678 / 12345678 bytes)";
        let progress = parse_progress_line(line);
        assert!(progress.is_some());
        let p = progress.unwrap();
        assert!(p.percentage.is_some());
    }

    #[test]
    fn test_parse_no_progress() {
        let line = "Login successful.";
        let progress = parse_progress_line(line);
        assert!(progress.is_none());
    }

    #[test]
    fn test_parse_speed() {
        let speed = parse_speed("1.5 MB/s");
        assert_eq!(speed, Some(1024 * 1024 + 512 * 1024));
    }

    #[test]
    fn test_parse_eta() {
        let eta = parse_eta("00:05:30");
        assert_eq!(eta, Some(330));
    }

    #[test]
    fn test_estimate_size() {
        let size = estimate_total_size(50_000_000, 50.0);
        assert_eq!(size, Some(100_000_000));
    }

    #[test]
    fn test_parse_kb_speed() {
        let speed = parse_speed("256 KB/s");
        assert_eq!(speed, Some(256 * 1024));
    }
}
