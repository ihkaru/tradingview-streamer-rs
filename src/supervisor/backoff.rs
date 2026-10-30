use std::time::Duration;

/// Exponential backoff calculator with optional full jitter to prevent thundering herd spikes.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    base_delay: Duration,
    max_delay: Duration,
    factor: f64,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            factor: 2.0,
        }
    }
}

impl ExponentialBackoff {
    #[must_use]
    pub fn new(base_delay: Duration, max_delay: Duration, factor: f64) -> Self {
        Self {
            base_delay,
            max_delay,
            factor: factor.max(1.1),
        }
    }

    /// Computes the backoff duration for the given attempt index (0-indexed).
    #[must_use]
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let mult = self.factor.powi(attempt.min(10) as i32);
        let exp_millis = (self.base_delay.as_millis() as f64) * mult;
        let clamped_millis = exp_millis.min(self.max_delay.as_millis() as f64) as u64;

        // Apply lightweight pseudo-random jitter between 50% and 100% of clamped duration
        let jitter_factor = 0.5 + (0.5 * (((attempt as f64 * 37.0) % 10.0) / 10.0));
        let final_millis = (clamped_millis as f64 * jitter_factor) as u64;

        Duration::from_millis(final_millis.max(self.base_delay.as_millis() as u64 / 2))
    }
}
