/// Service ID Filter
#[derive(Copy, Clone)]
pub struct SIDConfig {
    pub sft: u32,
    pub sfec: u32,
    pub sidf1: u32,
    pub sidf2: u32
}

impl SIDConfig {
    pub fn new() -> Self {
        Self {
            sft: 0,
            sfec: 0,
            sidf1: 0,
            sidf2: 0
        }
    }
}