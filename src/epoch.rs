

use ::std::time::Duration;

/// Provides a starting timestamp in nanoseconds from UNIX_EPOCH.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(Duration);

impl Epoch {
    /// Returns the current time as a UnixTimeStamp.
    #[inline]
    pub fn from_unix() -> Self {
        Self(::std::time::SystemTime::now().duration_since(::std::time::UNIX_EPOCH).unwrap())
    }

    /// Creates Epoch with a base duration.
    #[inline]
    pub fn from(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns a zeroed Epoch.
    #[inline]
    pub fn from_zero() -> Self {
        Self::from(Duration::new(0, 0))
    }

}

impl ::std::default::Default for Epoch {
    #[inline]
    fn default() -> Self {
        Self::from_unix()
    }
}

impl ::std::ops::Add for Epoch {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ::std::ops::Sub for Epoch {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ::std::ops::Add<Duration> for Epoch {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl ::std::ops::Sub<Duration> for Epoch {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl ::std::ops::AddAssign for Epoch {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ::std::ops::SubAssign for Epoch {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ::std::ops::AddAssign<Duration> for Epoch {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs;
    }
}

impl ::std::ops::SubAssign<Duration> for Epoch {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs;
    }
}


impl ::std::ops::Deref for Epoch {
    type Target = Duration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for Epoch {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ::std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0.as_secs() as f64 + self.0.subsec_nanos() as f64 * 1e-9)
    }
}

impl From<Duration> for Epoch {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl From<Epoch> for Duration {
    #[inline]
    fn from(epoch: Epoch) -> Self {
        epoch.0
    }
}
