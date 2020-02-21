use std::io::Write;

use metrics::{Metric, MetricWriter, DiffMetric};
use dummy_device::block_device::BlockMetrics;

#[derive(Default, Serialize)]
pub struct BlockMetricsImpl {
    successful_activation: DiffMetric,
    activate_error: DiffMetric,
    features_error: DiffMetric,
}

impl MetricWriter for BlockMetricsImpl {
    fn write(&self, buffer: &mut (dyn Write + Send)) {
        let msg = serde_json::to_string(&self).unwrap();
        let _ = buffer.write(msg.as_bytes());
        let _ = buffer.flush();
    }
}

impl BlockMetrics for BlockMetricsImpl {

    fn feature_error_inc(&self) {
        self.features_error.inc();
    }

    fn activate_error_inc(&self) {
        self.activate_error.inc();
    }

    fn successful_activation_inc(&self) {
        self.successful_activation.inc();
    }
}
