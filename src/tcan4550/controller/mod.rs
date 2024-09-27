pub mod configurator;

use std::borrow::Borrow;

pub struct TCAN455xController {}

impl TCAN455xController {

    const WRITE_B_FL: u8 = 0x61;
    const READ_B_FL: u8 = 0x41;

    pub(crate) fn generate_write_command <T: Borrow<Vec<u32>>> (addr: u16, data: T) -> Vec<u8>{
        let data = data.borrow();
        let addr: [u8; 2] = addr.to_be_bytes();
        let len: usize = data.len();
        let mut buf: Vec<u8> = Vec::<u8>::new();
        for i in 0..len {
            buf.extend(data[i].to_be_bytes());
        }

        let mut payload: Vec<u8> = Vec::new();
        payload.push(Self::WRITE_B_FL);
        payload.extend(addr);
        payload.push(len as u8);
        payload.extend(buf);
        
        payload
    }

    pub(crate) fn generate_read_command(addr: u16, len: u8) -> Vec<u8>{
        let addr: [u8; 2] = addr.to_be_bytes();
        /* zero padding in order to send sclk for extracting all MISO data*/
        let zero_padder: Vec<u8> = vec![0u8; 4 * len as usize];
        let mut payload: Vec<u8> = Vec::new(); 
        payload.push(Self::READ_B_FL);
        payload.extend(addr);
        payload.push(len as u8);
        payload.extend(&zero_padder);
        
        payload
    }
}
