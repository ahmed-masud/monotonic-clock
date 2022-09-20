//! #Monotonic Clocks
//!
//! This a convenience crate provides a monotonic clock for measuring
//! durations that can be anchored to a specific point in time.
//!
//! ## Example
//! ```
//! use monotonic_clock::MonotonicClock;
//! use std::thread;
//! use std::time::Duration;
//! let clock = MonotonicClock::new();
//! let start = clock.now();
//! thread::sleep(Duration::from_millis(100));
//! let end = clock.now();
//! assert!(end - start >= Duration::from_millis(100));
//! ```
#![deny(missing_docs)]

mod clock;
mod epoch;

pub use clock::MonotonicClock;
pub use epoch::Epoch;
