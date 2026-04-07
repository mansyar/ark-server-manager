//! Exponential backoff retry logic for operations.

use std::time::Duration;

/// Retry configuration.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of attempts.
    pub max_attempts: u32,
    /// Base delay between retries in milliseconds.
    pub base_delay_ms: u64,
    /// Maximum delay cap in milliseconds.
    pub max_delay_ms: u64,
    /// Multiplier for exponential backoff.
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create a new RetryConfig with custom max_attempts.
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Calculate delay for a given attempt number.
    pub fn delay_for(&self, attempt: u32) -> Duration {
        let delay_ms = self.base_delay_ms as f64 * self.multiplier.powi(attempt as i32 - 1);
        let delay_ms = delay_ms.min(self.max_delay_ms as f64);
        Duration::from_millis(delay_ms as u64)
    }
}

/// Retry a synchronous operation with exponential backoff.
pub fn retry<T, E, F: FnMut() -> Result<T, E>>(config: RetryConfig, mut f: F) -> Result<T, E> {
    let mut last_error = None;
    for attempt in 1..=config.max_attempts {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < config.max_attempts {
                    let delay = config.delay_for(attempt);
                    std::thread::sleep(delay);
                }
            }
        }
    }
    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.base_delay_ms, 1000);
        assert_eq!(config.multiplier, 2.0);
    }

    #[test]
    fn test_delay_calculation() {
        let config = RetryConfig::default();
        // Attempt 1: 1000ms
        assert_eq!(config.delay_for(1).as_millis(), 1000);
        // Attempt 2: 2000ms
        assert_eq!(config.delay_for(2).as_millis(), 2000);
        // Attempt 3: 4000ms
        assert_eq!(config.delay_for(3).as_millis(), 4000);
    }

    #[test]
    fn test_max_delay_cap() {
        let config = RetryConfig {
            max_delay_ms: 2000,
            ..Default::default()
        };
        // Attempt 3 should be capped at 2000ms (not 4000ms)
        assert_eq!(config.delay_for(3).as_millis(), 2000);
    }

    #[test]
    fn test_with_max_attempts() {
        let config = RetryConfig::default().with_max_attempts(5);
        assert_eq!(config.max_attempts, 5);
    }
}
