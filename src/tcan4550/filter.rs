#[derive(Copy, Clone)]
pub struct SIDFCONFIG {
    pub sft: u32,
    pub sfec: u32,
    pub sidf1: u32,
    pub sidf2: u32
}

impl SIDFCONFIG {
    pub fn new() -> Self {
        Self {
            sft: 0,
            sfec: 0,
            sidf1: 0,
            sidf2: 0
        }
    }
}

#[derive(Copy, Clone)]
pub struct XIDFCONFIG {
    pub eft: u32,
    pub efec: u32,
    pub eidf1: u32,
    pub eidf2: u32
}

impl XIDFCONFIG {
    pub fn new() -> Self {
        Self {
            eft: 0,
            efec: 0,
            eidf1: 0,
            eidf2: 0
        }
    }
}