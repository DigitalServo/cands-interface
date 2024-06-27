/// Extended ID Filter
#[derive(Copy, Clone)]
pub struct XIDConfig {
    pub eft: u32,
    pub efec: u32,
    pub eidf1: u32,
    pub eidf2: u32
}

impl XIDConfig {
    pub fn new() -> Self {
        Self {
            eft: 0,
            efec: 0,
            eidf1: 0,
            eidf2: 0
        }
    }
}