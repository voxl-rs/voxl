use std::time::Instant;

/// Use to keep track of delay, either for a
/// `System`'s state or as a component of an `Entity`.
#[derive(Debug, Clone, Copy)]
pub struct DeltaTime(Instant);

impl DeltaTime {
    /// Gets elapsed time since last call.
    pub fn elapsed(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }

    /// Gets the tps of this frame based on the elapsed time.
    pub fn tps(&self) -> f64 {
        1.0e09 / self.elapsed()
    }

    /// Captures a new instant to refer to for the next call.
    /// * For a system's state it should be called at the last line of the system.
    /// * For a component, it should be called at the end of each iteration.
    pub fn flush(&mut self) {
        self.0 = Instant::now();
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

#[derive(Debug)]
/// Counter that calculates the average tps over a collection of samples.
pub struct TpsCounter {
    pub dt: DeltaTime,
    buf: CircularBuffer<f64>,
    sum: f64,
}

impl TpsCounter {
    /// Create a new counter with a specified sample-size.
    pub fn new(samplesize: usize) -> Self {
        Self {
            dt: DeltaTime::default(),
            buf: CircularBuffer::<f64>::new(samplesize),
            sum: 0.,
        }
    }

    /// Get the average tps.
    pub fn tps(&self) -> f64 {
        if self.sum == 0. || self.buf.queue.is_empty() {
            return 0.0;
        }
        self.buf.queue.len() as f64 / self.sum
    }

    /// Pushes a new delta time value.
    /// Should be called in the first line of the system.
    pub fn update(&mut self) {
        let elem = self.dt.elapsed();
        self.sum += elem;

        if let Some(front) = self.buf.push(elem) {
            self.sum -= front;
        }
    }

    /// Captures a new instant to refer to for the next call.
    /// Should be called at the last line of the system.
    pub fn flush(&mut self) {
        self.dt.flush();
    }
}

impl Default for TpsCounter {
    fn default() -> Self {
        Self {
            dt: DeltaTime::default(),
            buf: CircularBuffer::new(16),
            sum: 0.,
        }
    }
}

use std::collections::VecDeque;
#[derive(Debug)]
struct CircularBuffer<T> {
    pub queue: VecDeque<T>,
    cap: usize,
}

impl<A> CircularBuffer<A> {
    /// Creates a new CircularBuffer with fixed size.
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            queue: VecDeque::with_capacity(size),
            cap: size,
        }
    }

    /// Add a value to the CircularBuffer.
    /// Returns the popped value if the buffer is full.
    pub fn push(&mut self, elem: A) -> Option<A> {
        let out = if self.queue.len() == self.cap {
            //front to back <-> oldest to newest
            self.queue.pop_front()
        } else {
            None
        };

        self.queue.push_back(elem);
        out
    }
}
