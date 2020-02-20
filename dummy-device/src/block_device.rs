use metrics::{Metric, MetricWriter};
use std::io::Write;

// This implementation allows users to create a `BlockDevice` without metrics.
impl BlockMetrics for () {
    fn feature_error(&self) -> Box<&dyn Metric> {
        Box::new(self)
    }

    fn activate_error(&self) -> Box<&dyn Metric> {
        Box::new(self)
    }

    fn successful_activation(&self) -> Box<&dyn Metric> {
        Box::new(self)
    }
}

/// Defines metrics specific to the block device implementation.
pub trait BlockMetrics : MetricWriter {
    fn feature_error(&self) -> Box<&dyn Metric>;
    fn activate_error(&self) -> Box<&dyn Metric>;
    fn successful_activation(&self) -> Box<&dyn Metric>;
}

pub struct BlockDevice<T: BlockMetrics> {
    features: u64,
    // Generic type that represents the block metrics. When users don't want
    // metrics to be enabled, BlockDevice can be instantiated with `NoOpMetric`.
    metrics: T,
    activate_success: bool,
}

impl BlockDevice <()> {
    pub fn new() -> BlockDevice<()> {
        BlockDevice {
            features: 0,
            metrics: (),
            activate_success: false,
        }
    }
}

impl <T: BlockMetrics + Default> BlockDevice<T> {
    pub fn new_with_metrics() -> BlockDevice<T> {
        BlockDevice {
            features: 0,
            metrics: T::default(),
            activate_success: false,
        }
    }

    pub fn avail_feature(&self) -> u64 {
        123
    }

    pub fn set_features(&mut self, val: u64) {
        if val == 0 {
            self.metrics.feature_error().inc();
            return;
        }
        self.features = val;
    }

    pub fn activate(&self) {
        if self.activate_success {
            self.metrics.successful_activation().inc();
        } else {
            self.metrics.activate_error().inc();
        }
    }

    pub fn flush_metrics(&self, buf: &mut (dyn Write + Send)) {
        self.metrics.write(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::io::Write;

    #[derive(Default, Serialize)]
    struct BlockMetricsImpl {
        successful_activation: AtomicUsize,
        activate_error: AtomicUsize,
        features_error: AtomicUsize,
    }

    impl MetricWriter for BlockMetricsImpl {
        fn write(&self, buffer: &mut (dyn Write + Send)) {
            let msg = serde_json::to_string(&self).unwrap();
            let _ = buffer
                .write(msg.as_bytes());
            let _ = buffer.flush();
        }
    }

    impl BlockMetrics for BlockMetricsImpl {

        fn feature_error(&self) -> Box<&dyn Metric> {
            Box::new(&self.features_error)
        }

        fn activate_error(&self) -> Box<&dyn Metric> {
            Box::new(&self.activate_error)
        }

        fn successful_activation(&self) -> Box<&dyn Metric> {
            Box::new(&self.successful_activation)
        }
    }

    #[test]
    fn test_no_op_metric() {
        let block_device = BlockDevice::new();
        block_device.activate();
    }

    #[test]
    fn test_metrics() {
        let mut block_device = BlockDevice::<BlockMetricsImpl>::new_with_metrics();
        assert_eq!(block_device.metrics.features_error.count(), 0);
        assert_eq!(block_device.metrics.activate_error.count(), 0);
        assert_eq!(block_device.metrics.successful_activation.count(), 0);

        block_device.activate_success = true;
        block_device.activate();
        assert_eq!(block_device.metrics.successful_activation.count(), 1);
        assert_eq!(block_device.metrics.activate_error.count(), 0);
    }
}