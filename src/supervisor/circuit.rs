use std::time::{Duration, Instant};

/// Operational state of the circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation: requests pass through.
    Closed,
    /// Breaker tripped: requests fail fast to allow upstream recovery.
    Open,
    /// Recovery probe: single request allowed to test upstream health.
    HalfOpen,
}

/// Dynamic circuit breaker guarding against persistent upstream HTTP 429/500 errors.
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    cooldown_duration: Duration,
    consecutive_failures: usize,
    state: CircuitState,
    last_state_change: Instant,
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            cooldown_duration: Duration::from_secs(30),
            consecutive_failures: 0,
            state: CircuitState::Closed,
            last_state_change: Instant::now(),
        }
    }
}

impl CircuitBreaker {
    #[must_use]
    pub fn new(failure_threshold: usize, cooldown_duration: Duration) -> Self {
        Self {
            failure_threshold: failure_threshold.max(1),
            cooldown_duration,
            consecutive_failures: 0,
            state: CircuitState::Closed,
            last_state_change: Instant::now(),
        }
    }

    /// Evaluates if an operation is permitted to execute.
    pub fn can_execute(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if self.last_state_change.elapsed() >= self.cooldown_duration {
                    self.state = CircuitState::HalfOpen;
                    self.last_state_change = Instant::now();
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Records a successful operation, resetting failure counters.
    pub fn record_success(&mut self) {
        self.consecutive_failures = 0;
        self.state = CircuitState::Closed;
        self.last_state_change = Instant::now();
    }

    /// Records a failed operation, transitioning states if failure threshold is reached.
    pub fn record_failure(&mut self) -> CircuitState {
        self.consecutive_failures += 1;
        if self.consecutive_failures >= self.failure_threshold || self.state == CircuitState::HalfOpen {
            self.state = CircuitState::Open;
            self.last_state_change = Instant::now();
        }
        self.state
    }

    #[must_use]
    pub fn state(&self) -> CircuitState {
        self.state
    }

    #[must_use]
    pub fn failures(&self) -> usize {
        self.consecutive_failures
    }
}
