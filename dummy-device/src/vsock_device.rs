#![allow(dead_code)]
#![allow(unused)]

struct VsockDevice {
    features: u64,
}

impl VsockDevice {
    pub fn avail_feature(&self) -> u64 {
        76
    }

    pub fn set_features(&mut self, val: u64) {
        if val == 0 {
            // increment metric
        }

        self.features = val;
    }

    pub fn activate(&self) {
        // something with metrics as well
    }
}