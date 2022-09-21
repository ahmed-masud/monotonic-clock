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
/// use monotonic_clock::Clock;
/// use std::thread;
/// use std::time::Duration;
/// let clock = Clock::new();
/// let start = clock.now();
/// thread::sleep(Duration::from_millis(100));
/// let end = clock.now();
/// assert!(end - start >= Duration::from_millis(100));
/// ```
///

pub trait MonotonicClock {
    /// Return the epoch of the clock.
    fn epoch(&self) -> Epoch;

    /// Returns the current time.
    fn now(&self) -> Duration;

    /// Returns true if the clock is ticking.
    fn is_ticking(&self) -> bool;

    /// Get the now time since the epoch.
    #[inline]
    fn time(&self) -> Duration {
        self.now() + *self.epoch()
    }

    /// Get the now time since the epoch as a float.
    #[inline]
    fn time_as_float(&self) -> f64 {
        let time = self.time();
        time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
    }

    /// Get the now time since the epoch as a float.
    /// This is a convenience function for `clock_as_float`.
    /// It is provided for compatibility with the `time` crate.
    #[inline]
    fn as_float(&self) -> f64 {
        self.time_as_float()
    }
}

/// A monotonic clock that can be anchored to a specific [Epoch].

#[derive(Debug, Clone)]
pub struct Clock {
    inner: ::std::sync::Arc<::std::sync::RwLock<InnerClock>>,
}

unsafe impl Sync for Clock {}
unsafe impl Send for Clock {}

impl Clock {
    /// Create a new clock.
    pub fn new() -> Self {
        Self {
            inner: ::std::sync::Arc::new(::std::sync::RwLock::new(InnerClock::new())),
        }
    }

    /// Start the clock.
    pub fn start(&self) {
        self.inner.write().unwrap().start();
    }

    /// Stop the clock.
    pub fn stop(&self) -> Option<Duration> {
        self.inner.write().unwrap().stop()
    }

    /// Reset the clock.
    pub fn reset(&self) {
        self.inner.write().unwrap().reset();
    }

    /// Resume a paused clock.
    pub fn resume(&self) -> Option<Duration> {
        self.inner.write().unwrap().resume()
    }
}

impl MonotonicClock for Clock {
    fn epoch(&self) -> Epoch {
        self.inner.read().unwrap().epoch()
    }

    fn now(&self) -> Duration {
        self.inner.read().unwrap().now()
    }

    fn is_ticking(&self) -> bool {
        self.inner.read().unwrap().is_ticking()
    }
}

impl Default for Clock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InnerClock {
    epoch: Epoch, // The unix_epoch time at which the clock was created.
    start: ::std::time::Instant,
    stop: Option<::std::time::Instant>,
}

impl InnerClock {
    /// Create a new monotonic clock.
    #[inline]
    pub fn new() -> Self {
        Self {
            epoch: Epoch::from_unix(),
            start: ::std::time::Instant::now(),
            stop: None,
        }
    }

    /// Returns the epoch of the clock.
    #[inline]
    pub fn epoch(&self) -> Epoch {
        self.epoch
    }

    /// Reset the clock to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.epoch = Epoch::from_unix();
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

    /// Is the clock running?
    #[inline]
    pub fn is_ticking(&self) -> bool {
        self.stop.is_none()
    }
}

impl Default for InnerClock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MonotonicClock for InnerClock {
    #[inline]
    fn epoch(&self) -> Epoch {
        self.epoch
    }

    #[inline]
    fn now(&self) -> Duration {
        self.now()
    }

    #[inline]
    fn is_ticking(&self) -> bool {
        self.is_ticking()
    }
}

impl ::std::fmt::Display for InnerClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.time_as_float())
    }
}

impl ::std::convert::From<InnerClock> for Duration {
    /// Get the now time since the clock's epoch.
    fn from(mc: InnerClock) -> Self {
        mc.time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    #[test]
    fn test_monotonic_clock() {
        let clock = Clock::new();
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
        let clock = Clock::new();
        eprintln!("clock.epoch = {:?}", clock.epoch());
        eprintln!("clock.now() = {:?}", clock.time());
    }
}
