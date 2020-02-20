use std::io::Write;
use std::sync::atomic::AtomicUsize;

use metrics::{Metric, MetricWriter};
use dummy_device::block_device::BlockMetrics;

#[derive(Default, Serialize)]
pub struct BlockMetricsImpl {
    successful_activation: AtomicUsize,
    activate_error: AtomicUsize,
    features_error: AtomicUsize,
}

impl MetricWriter for BlockMetricsImpl {
    fn write(&self, buffer: &mut (dyn Write + Send)) {
        let msg = serde_json::to_string(&self).unwrap();
        let _ = buffer.write(msg.as_bytes());
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
