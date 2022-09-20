use crate::epoch::Epoch;
use ::std::time::Duration;

/// # Monotonic Clock
///  A monotonic clock that can be anchored to a specific [Epoch].
/// The clock is guaranteed to be monotonic, but not necessarily
/// continuous.
///
///
/// ## Thread safety
/// The clock is thread safe.
///
/// Eventually, we want to have network synchronization, but for now, we
/// just use the system clock.
/// TODO: Add network synchronization.
///
/// ## Example
/// ```
/// use monotonic_clock::MonotonicClock;
/// use std::thread;
/// use std::time::Duration;
/// let clock = MonotonicClock::new();
/// let start = clock.now();
/// thread::sleep(Duration::from_millis(100));
/// let end = clock.now();
/// assert!(end - start >= Duration::from_millis(100));
/// ```
///

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonotonicClock {
    epoch: Epoch, // The unix_epoch time at which the clock was created.
    start: ::std::time::Instant,
    stop: Option<::std::time::Instant>,
}

impl Default for MonotonicClock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MonotonicClock {
    /// Create a new monotonic clock.
    #[inline]
    pub fn new() -> MonotonicClock {
        MonotonicClock {
            epoch: Epoch::from_unix(),
            start: ::std::time::Instant::now(),
            stop: None,
        }
    }

    /// Reset the clock to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.start = ::std::time::Instant::now();
        self.stop = None;
    }

    /// Start the clock.
    #[inline]
    pub fn start(&mut self) {
        self.start = ::std::time::Instant::now();
        self.stop = None;
    }

    /// Resumes paused clock.
    /// If the clock is not stopped, this does nothing.

    #[inline]
    pub fn resume(&mut self) -> Option<Duration> {
        if let Some(stop) = self.stop {
            self.stop = None;
            ::std::time::Instant::now().checked_duration_since(stop)
        } else {
            Some(Duration::new(0, 0))
        }
    }

    /// Stop the clock if it's running, otherwise does nothing.
    /// Returns the duration the clock was running.
    #[inline]
    pub fn stop(&mut self) -> Option<Duration> {
        if self.stop.is_none() {
            self.stop = Some(::std::time::Instant::now());
        }
        self.stop.map(|stop| stop - self.start)
    }

    /// Get duration since the clock has been running time.
    #[inline]
    pub fn now(&self) -> Duration {
        if let Some(stop) = self.stop {
            stop.duration_since(self.start)
        } else {
            ::std::time::Instant::now().duration_since(self.start)
        }
    }

    /// Get the now time since the epoch.
    #[inline]
    pub fn time(&self) -> Duration {
        self.now() + *self.epoch
    }

    /// Get the now time since the epoch as a float.
    #[inline]
    pub fn time_as_float(&self) -> f64 {
        let time = self.time();
        time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
    }

    /// Get the now time since the epoch as a float.
    /// This is a convenience function for `clock_as_float`.
    /// It is provided for compatibility with the `time` crate.
    #[inline]
    pub fn as_float(&self) -> f64 {
        self.time_as_float()
    }

    /// Is the clock running?
    #[inline]
    pub fn is_ticking(&self) -> bool {
        self.stop.is_none()
    }
}

impl ::std::fmt::Display for MonotonicClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.time_as_float())
    }
}

impl From<MonotonicClock> for Duration {
    /// Get the now time since the clock's epoch.
    fn from(mc: MonotonicClock) -> Self {
        mc.time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    #[test]
    fn test_monotonic_clock() {
        let mut clock = MonotonicClock::new();
        assert!(clock.now() < Duration::from_secs(1));
        ::std::thread::sleep(Duration::from_secs(1));
        assert!(clock.now() > Duration::from_secs(1));
        ::std::thread::sleep(Duration::from_secs(2));

        let stopped_at = clock.stop().unwrap();
        assert!(clock.now() > Duration::from_secs(2));
        assert!(clock.now() == stopped_at);

        ::std::thread::sleep(Duration::from_secs(2));
        assert!(clock.now() > Duration::from_secs(2));
        assert!(clock.now() == stopped_at);

        clock.resume();
        ::std::thread::sleep(Duration::from_secs(1));
        assert!(clock.now() > stopped_at);
        clock.reset();
        assert!(clock.now() < Duration::from_secs(1));
    }

    #[test]
    fn test_monotonic_clock_since_unix_epoch() {
        let clock = MonotonicClock::new();
        eprintln!("clock.epoch = {:?}", clock.epoch);
        eprintln!("clock.now() = {:?}", clock.time());
    }
}
