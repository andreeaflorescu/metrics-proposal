use std::io::Write;

use std::sync::atomic::{AtomicUsize, Ordering};

pub trait Metric {
    /// Adds `value` to the current counter.
    fn add(&self, value: usize);
    /// Increments by 1 unit the current counter.
    fn inc(&self) {
        self.add(1);
    }
    /// Returns current value of the counter.
    fn count(&self) -> usize;
}

pub trait MetricWriter {
    fn write(&self, buffer: &mut (dyn Write + Send));
}

impl MetricWriter for () {
    fn write(&self, _: &mut (dyn Write + Send)) {}
}

impl Metric for () {
    fn add(&self, _: usize) { }

    fn count(&self) -> usize {
        0
    }
}

pub struct DiffMetric {
    current_value: AtomicUsize,
    previous_value: AtomicUsize,
}

impl Metric for DiffMetric {
    fn add(&self, value: usize) {
        self.current_value.fetch_add(value, Ordering::Relaxed);
    }

    fn count(&self) -> usize {
        self.current_value.load(Ordering::Relaxed)
    }
}

impl Metric for AtomicUsize {
    /// Adds `value` to the current counter.
    fn add(&self, value: usize) {
        self.fetch_add(value, Ordering::Relaxed);
    }

    /// Returns current value of the counter.
    fn count(&self) -> usize {
        self.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_works() {
        println!("blah");
    }
}