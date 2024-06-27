const FIFOSIZE: usize = 1024;

/// Receive data buffer on user space
#[derive(Debug)]
pub struct RxData {
    pub fifo0: Vec<u8>,
    pub fifo1: Vec<u8>
}

impl RxData {
    pub fn new() -> Self {
        Self {
            fifo0: Vec::with_capacity(FIFOSIZE),
            fifo1: Vec::with_capacity(FIFOSIZE),
        }
    }

    pub fn reset(&mut self) {
        self.fifo0.clear();
        self.fifo1.clear();
    }
}
