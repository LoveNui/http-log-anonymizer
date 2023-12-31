//! Module with helper components for keeping track of the limit for a rate-limited operation.
//!
use std::time::{Duration, Instant};

// XXX: make the limiter adaptive (learn an appropriate request rate from an init setting)
/// Component responsible for tracking the time since last recorded request and ensuring that it
/// does not exceed specified limit.
pub struct RequestLimiter {
    /// The rate with each request can be sent in seconds.
    request_rate: Duration,
    /// The timestamp of the last issued request.
    last_request: Instant,
}

impl RequestLimiter {
    /// Craete new limiter for given `request_rate` in seconds.
    ///
    /// Panics if `request_rate` is zero.
    #[inline]
    pub fn new(request_rate: Duration) -> Self {
        assert!(
            !request_rate.is_zero(),
            "request rate cannot be zero, got: {request_rate:?}",
        );
        Self {
            request_rate,
            last_request: Instant::now(),
        }
    }

    /// Returns the time left in seconds till the next request can be made, saturates at 0.
    #[inline]
    pub fn remaining_time(&self) -> Duration {
        self.request_rate
            .saturating_sub(self.last_request.elapsed())
    }

    /// Resets the limiter to current timestamp.
    #[inline]
    pub fn record_request(&mut self) {
        self.last_request = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_tracks_time_left() {
        let mut limiter = RequestLimiter::new(Duration::from_secs(2));

        let time_left = limiter.remaining_time();
        assert!(time_left > Duration::from_secs(0));

        std::thread::sleep(time_left);
        let time_left = limiter.remaining_time();
        assert_eq!(time_left, Duration::from_secs(0));

        limiter.record_request();
        let time_left = limiter.remaining_time();
        assert!(time_left > Duration::from_secs(0));
    }

    #[test]
    #[should_panic(expected = "request rate cannot be zero")]
    fn panics_for_zero_rate() {
        RequestLimiter::new(Default::default());
    }
}
