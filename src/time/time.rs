use std::time::Instant;

/// Use this to keep track of the delay between calls of a system
#[derive(Debug, Clone, Copy)]
pub struct DeltaTime(Instant);

impl DeltaTime {
    /// Gets elapsed time since last call.
    #[inline(always)]
    pub fn val(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }

    /// Recaptures the instant to be used again.
    /// Must be called at the last line of the system.
    #[inline(always)]
    pub fn flush(&mut self) {
        self.0 = Instant::now();
    }

    /// Ticks per second
    #[inline(always)]
    pub fn tps(&mut self) -> f64 {
        1f64 / self.val()
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}
