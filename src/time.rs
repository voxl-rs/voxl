use std::time::Instant;

/// Use this to keep track of the delay between calls of a system;
/// If you want interpolated values based from a specified sample size
/// use `FpsCounter` instead.
#[derive(Debug, Clone, Copy)]
pub struct DeltaTime(Instant);

impl DeltaTime {
    /// Gets elapsed time since last call.
    #[inline(always)]
    pub fn elapsed(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }

    /// Renews the instant for the next call;
    /// Must be called at the last line of the system.
    #[inline(always)]
    pub fn flush(&mut self) {
        self.0 = Instant::now();
    }

    /// Ticks per second
    #[inline(always)]
    pub fn tps(&self) -> f64 {
        1.0e09 / self.elapsed()
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

#[derive(Debug)]
pub struct FpsCounter {
    pub dt: DeltaTime,
    buf: CircularBuffer<f64>,
    sum: f64,
}

impl FpsCounter {
    /// Creates a new FpsCounter that calculates the average fps over samplesize values.
    pub fn new(samplesize: usize) -> Self {
        Self {
            dt: DeltaTime::default(),
            buf: CircularBuffer::<f64>::new(samplesize),
            sum: 0.,
        }
    }

    /// Add a new delta time value.
    /// Must be called in the first line of the system.
    pub fn update(&mut self) {
        let elem = self.dt.elapsed();
        self.sum += elem;

        if let Some(front) = self.buf.push(elem) {
            self.sum -= front;
        }
    }

    ///Get the average fps over the samplesize frames.
    pub fn tps(&self) -> f64 {
        if self.sum == 0. || self.buf.queue.is_empty() {
            return 0.0;
        }
        /*1.0e9 */
        self.buf.queue.len() as f64 / self.sum
    }

    /// Resets for recording a new time delta for the next call;
    /// Must be called at the last line of the system.
    #[inline(always)]
    pub fn flush(&mut self) {
        self.dt.flush();
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            dt: DeltaTime::default(),
            buf: CircularBuffer::new(32),
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
    ///Creates a new CircularBuffer with fixed size
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            queue: VecDeque::with_capacity(size),
            cap: size,
        }
    }

    ///Add a value to the CircularBuffer
    ///Returns the popped value if the buffer is full
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
