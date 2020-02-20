#[macro_use]
extern crate serde_derive;

mod vmm_metrics;

use dummy_device::block_device::BlockDevice;
use vmm_metrics::BlockMetricsImpl;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;


struct Vmm {
    block_device: BlockDevice<BlockMetricsImpl>,
    writer: Box<dyn Write + Send>,
}

impl Vmm {
    pub fn new() -> Self {
        let dest = PathBuf::from("blahblah");
        let dest = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&dest).unwrap();

        let block_device = BlockDevice::<BlockMetricsImpl>::new_with_metrics();

        Vmm {
            block_device,
            writer: Box::new(dest),
        }
    }

    pub fn flush_metrics(&mut self) {
        self.block_device.flush_metrics(&mut self.writer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vmm = Vmm::new();
        vmm.flush_metrics();
        vmm.block_device.activate();
        vmm.flush_metrics();
    }
}
