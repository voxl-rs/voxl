#[deny(missing_docs)]
use std::time::Instant;

/// A state for a system to be used to retrieve the time it(system) was last invoked.
pub struct DeltaTime(Instant);

impl DeltaTime {
    /// Gets elapsed time since last call.
    #[inline]
    pub fn val(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }

    /// Recaptures the instant to be used again.
    /// Must be called at the last line of the system.
    #[inline]
    pub fn flush(&mut self) {
        self.0 = Instant::now();
    }

    /// Ticks per second
    #[inline]
    pub fn tps(&mut self) -> f64 {
        1f64 / self.val()
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}
