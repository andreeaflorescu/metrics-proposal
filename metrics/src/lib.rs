use std::io::Write;

use serde::{Serialize, Serializer};

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
    /// Resets the inner counter.
    fn reset(&self) {}
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

#[derive(Default)]
pub struct DiffMetric {
    current_value: AtomicUsize,
    previous_value: AtomicUsize,
}

impl Serialize for DiffMetric {
    /// Reset counters of each metrics. Here we suppose that Serialize's goal is to help with the
    /// flushing of metrics.
    /// !!! Any print of the metrics will also reset them. Use with caution !!!
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // There's no serializer.serialize_usize() for some reason :(
        let snapshot = self.current_value.load(Ordering::Relaxed);
        let res = serializer.serialize_u64(snapshot as u64 - self.previous_value.load(Ordering::Relaxed) as u64);

        if res.is_ok() {
            self.previous_value.store(snapshot, Ordering::Relaxed);
        }
        res
    }
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
