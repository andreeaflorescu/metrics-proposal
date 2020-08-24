use metrics::Metric;

struct BlockDeviceMetrics2<T: Metric + Default> {
    pub (crate) feature_error: T,
    pub (crate) activate_error: T,
    pub (crate) successful_activation: T,
}

pub struct BlockDevice<T: Metric + Default> {
    // Generic type that represents the block metrics. When users don't want
    // metrics to be enabled, BlockDevice can be instantiated with `()`.
    metrics: BlockDeviceMetrics2<T>,
    features: u64,
    activate_success: bool,
}

impl BlockDevice <()> {
    pub fn new() -> BlockDevice<()> {
        BlockDevice {
            features: 0,
            metrics: BlockDeviceMetrics2::<()>::new(),
            activate_success: false,
        }
    }
}

impl<T: Metric + Default> BlockDeviceMetrics2<T> {
    fn new() -> Self {
        Self {
            activate_error: T::default(),
            feature_error: T::default(),
            successful_activation: T::default(),
        }
    }
}
impl <T: Metric + Default> BlockDevice<T> {
    pub fn new_with_metrics() -> BlockDevice<T> {
        BlockDevice {
            features: 0,
            metrics: BlockDeviceMetrics2::<T>::new(),
            activate_success: false,
        }
    }

    pub fn avail_feature(&self) -> u64 {
        123
    }

    pub fn set_features(&mut self, val: u64) {
        if val == 0 {
            self.metrics.feature_error.inc();
            return;
        }
        self.features = val;
    }

    pub fn activate(&self) {
        if self.activate_success {
            self.metrics.successful_activation.inc();
        } else {
            self.metrics.activate_error.inc();
        }
    }

    // pub fn flush_metrics(&self, buf: &mut (dyn Write + Send)) {
    //     self.metrics.write(buf);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_no_op_metric() {
        let block_device = BlockDevice::new();
        block_device.activate();
    }

    #[test]
    fn test_metrics() {
        let mut block_device = BlockDevice::<AtomicUsize>::new_with_metrics();
        assert_eq!(block_device.metrics.feature_error.count(), 0);
        assert_eq!(block_device.metrics.activate_error.count(), 0);
        assert_eq!(block_device.metrics.successful_activation.count(), 0);

        block_device.activate_success = true;
        block_device.activate();
        assert_eq!(block_device.metrics.successful_activation.count(), 1);
        assert_eq!(block_device.metrics.activate_error.count(), 0);
    }
}
