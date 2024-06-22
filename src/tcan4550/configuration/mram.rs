use crate::tcan4550::register::*;
use crate::tcan4550::filter::{SIDFCONFIG, XIDFCONFIG};

// MRAM sections
const MRAM_SECTIONS_NUM: usize = 7;

#[derive(Copy, Clone)]
pub struct FIFODATASIZE {
    pub size: u32,
    pub code: u32,
}

// MRAM size
pub const FIFODATASIZE_LIST: [FIFODATASIZE; 8] = [
    FIFODATASIZE {size: 8,  code: 0b000},
    FIFODATASIZE {size: 12, code: 0b001},
    FIFODATASIZE {size: 16, code: 0b010},
    FIFODATASIZE {size: 20, code: 0b011},
    FIFODATASIZE {size: 24, code: 0b100},
    FIFODATASIZE {size: 32, code: 0b101},
    FIFODATASIZE {size: 48, code: 0b110},
    FIFODATASIZE {size: 64, code: 0b111},
];

pub const RXFIFO0DATASIZE: FIFODATASIZE = FIFODATASIZE_LIST[7];
pub const RXFIFO1DATASIZE: FIFODATASIZE = FIFODATASIZE_LIST[7];
pub const RXBCDATASIZE: FIFODATASIZE = FIFODATASIZE_LIST[0];
pub const TXFIFODATASIZE: FIFODATASIZE = FIFODATASIZE_LIST[7];

pub const MRAMCONFIG_BYTESPERELEMENT_SID: u32 = 4;
pub const MRAMCONFIG_BYTESPERELEMENT_XID: u32 = 8;
pub const MRAMCONFIG_BYTESPERELEMENT_RXFIFO0: u32 = RXFIFO0DATASIZE.size + 8;
pub const MRAMCONFIG_BYTESPERELEMENT_RXFIFO1: u32 = RXFIFO1DATASIZE.size + 8;
pub const MRAMCONFIG_BYTESPERELEMENT_RXBC: u32 =  0;
pub const MRAMCONFIG_BYTESPERELEMENT_TXEFC: u32 =  8;
pub const MRAMCONFIG_BYTESPERELEMENT_TXBC: u32 =  TXFIFODATASIZE.size + 8;

pub const MRAMCONFIG_NUMOFELEMENTS_SID: u32 = 2;
pub const MRAMCONFIG_NUMOFELEMENTS_XID: u32 = 1;
pub const MRAMCONFIG_NUMOFELEMENTS_RXFIFO0: u32 = 4;
pub const MRAMCONFIG_NUMOFELEMENTS_RXFIFO1: u32 = 5;
pub const MRAMCONFIG_NUMOFELEMENTS_RXBC: u32 = 0;
pub const MRAMCONFIG_NUMOFELEMENTS_TXEFC: u32 = 3;
pub const MRAMCONFIG_NUMOFELEMENTS_TXBC: u32 = 10;

pub const MRAMCONFIG_BYTESPERELEMENT: [u32; MRAM_SECTIONS_NUM] = [
    MRAMCONFIG_BYTESPERELEMENT_SID,
    MRAMCONFIG_BYTESPERELEMENT_XID,
    MRAMCONFIG_BYTESPERELEMENT_RXFIFO0,
    MRAMCONFIG_BYTESPERELEMENT_RXFIFO1,
    MRAMCONFIG_BYTESPERELEMENT_RXBC,
    MRAMCONFIG_BYTESPERELEMENT_TXEFC,
    MRAMCONFIG_BYTESPERELEMENT_TXBC,
];

pub const MRAMCONFIG_NUMOFELEMENTS: [u32; MRAM_SECTIONS_NUM] = [
    MRAMCONFIG_NUMOFELEMENTS_SID,
    MRAMCONFIG_NUMOFELEMENTS_XID,
    MRAMCONFIG_NUMOFELEMENTS_RXFIFO0,
    MRAMCONFIG_NUMOFELEMENTS_RXFIFO1,
    MRAMCONFIG_NUMOFELEMENTS_RXBC,
    MRAMCONFIG_NUMOFELEMENTS_TXEFC,
    MRAMCONFIG_NUMOFELEMENTS_TXBC,
];

pub const MRAMCONFIG_MEMSIZE: [u32; MRAM_SECTIONS_NUM] = [
    MRAMCONFIG_NUMOFELEMENTS_SID * MRAMCONFIG_BYTESPERELEMENT_SID,
    MRAMCONFIG_NUMOFELEMENTS_XID * MRAMCONFIG_BYTESPERELEMENT_XID,
    MRAMCONFIG_NUMOFELEMENTS_RXFIFO0 * MRAMCONFIG_BYTESPERELEMENT_RXFIFO0,
    MRAMCONFIG_NUMOFELEMENTS_RXFIFO1 * MRAMCONFIG_BYTESPERELEMENT_RXFIFO1,
    MRAMCONFIG_NUMOFELEMENTS_RXBC * MRAMCONFIG_BYTESPERELEMENT_RXBC,
    MRAMCONFIG_NUMOFELEMENTS_TXEFC * MRAMCONFIG_BYTESPERELEMENT_TXEFC,
    MRAMCONFIG_NUMOFELEMENTS_TXBC * MRAMCONFIG_BYTESPERELEMENT_TXBC,
];

// MRAM address
pub const MRAM_BASEADDR: u16 = 0x8000;

pub const MRAM_OFFSETADDR: [u16; MRAM_SECTIONS_NUM] = get_mram_offset_addrs();
pub const MRAM_OFFSETADDR_SID: u16 = MRAM_OFFSETADDR[0];
pub const MRAM_OFFSETADDR_XID: u16 = MRAM_OFFSETADDR[1];
pub const MRAM_OFFSETADDR_RXFIFO0: u16 = MRAM_OFFSETADDR[2];
pub const MRAM_OFFSETADDR_RXFIFO1: u16 = MRAM_OFFSETADDR[3];
pub const MRAM_OFFSETADDR_RXBC: u16 = MRAM_OFFSETADDR[4];
pub const MRAM_OFFSETADDR_TXEFC: u16 = MRAM_OFFSETADDR[5];
pub const MRAM_OFFSETADDR_TXBC: u16 = MRAM_OFFSETADDR[6];

pub const MRAM_STARTADDR: [u16; MRAM_SECTIONS_NUM] = get_mram_start_addrs(MRAM_OFFSETADDR);
pub const MRAM_STARTADDR_SID: u16 = MRAM_STARTADDR[0];
pub const MRAM_STARTADDR_XID: u16 = MRAM_STARTADDR[1];
pub const MRAM_STARTADDR_RXFIFO0: u16 = MRAM_STARTADDR[2];
pub const MRAM_STARTADDR_RXFIFO1: u16 = MRAM_STARTADDR[3];
pub const MRAM_STARTADDR_RXBC: u16 = MRAM_STARTADDR[4];
pub const MRAM_STARTADDR_TXEFC: u16 = MRAM_STARTADDR[5];
pub const MRAM_STARTADDR_TXBC: u16 = MRAM_STARTADDR[6];

pub const RXDATA_BLOCKSIZE: [u32; 2] = [MRAMCONFIG_BYTESPERELEMENT_RXFIFO0, MRAMCONFIG_BYTESPERELEMENT_RXFIFO1];
pub const RXDATA_FIFOSIZE: [u32; 2] = [MRAMCONFIG_NUMOFELEMENTS_RXFIFO0, MRAMCONFIG_NUMOFELEMENTS_RXFIFO1];

const fn get_mram_offset_addrs() -> [u16; MRAM_SECTIONS_NUM] {
    let mut mram_start_addrs: [u16; MRAM_SECTIONS_NUM] = [0; MRAM_SECTIONS_NUM];

    mram_start_addrs[0] = 0;
    mram_start_addrs[1] = mram_start_addrs[0] + MRAMCONFIG_MEMSIZE[0] as u16;
    mram_start_addrs[2] = mram_start_addrs[1] + MRAMCONFIG_MEMSIZE[1] as u16;
    mram_start_addrs[3] = mram_start_addrs[2] + MRAMCONFIG_MEMSIZE[2] as u16;
    mram_start_addrs[4] = mram_start_addrs[3] + MRAMCONFIG_MEMSIZE[3] as u16;
    mram_start_addrs[5] = mram_start_addrs[4] + MRAMCONFIG_MEMSIZE[4] as u16;
    mram_start_addrs[6] = mram_start_addrs[5] + MRAMCONFIG_MEMSIZE[5] as u16;

    if MRAMCONFIG_NUMOFELEMENTS[0] == 0 { mram_start_addrs[0] = 0; }
    if MRAMCONFIG_NUMOFELEMENTS[1] == 0 { mram_start_addrs[1] = 0; }
    if MRAMCONFIG_NUMOFELEMENTS[2] == 0 { mram_start_addrs[2] = 0; }
    if MRAMCONFIG_NUMOFELEMENTS[3] == 0 { mram_start_addrs[3] = 0; }
    if MRAMCONFIG_NUMOFELEMENTS[4] == 0 { mram_start_addrs[4] = 0; }
    if MRAMCONFIG_NUMOFELEMENTS[5] == 0 { mram_start_addrs[5] = 0; }
    if MRAMCONFIG_NUMOFELEMENTS[6] == 0 { mram_start_addrs[6] = 0; }

    mram_start_addrs
}

const fn get_mram_start_addrs(offset_addr: [u16; MRAM_SECTIONS_NUM]) -> [u16; MRAM_SECTIONS_NUM] {
    let mut mram_start_addrs: [u16; MRAM_SECTIONS_NUM] = [0; MRAM_SECTIONS_NUM];

    mram_start_addrs[0] = MRAM_BASEADDR + offset_addr[0];
    mram_start_addrs[1] = MRAM_BASEADDR + offset_addr[1];
    mram_start_addrs[2] = MRAM_BASEADDR + offset_addr[2];
    mram_start_addrs[3] = MRAM_BASEADDR + offset_addr[3];
    mram_start_addrs[4] = MRAM_BASEADDR + offset_addr[4];
    mram_start_addrs[5] = MRAM_BASEADDR + offset_addr[5];
    mram_start_addrs[6] = MRAM_BASEADDR + offset_addr[6];

    mram_start_addrs
}

// Water interrupt
pub const RXFIFO_WM_MAX: u32 = 64;
pub const TXEFC_WM_MAX: u32 = 32;
pub const RXFIFO0_WM: u32 = 0;
pub const RXFIFO1_WM: u32 = 0;
pub const TXEFC_WM: u32 = 2;

impl crate::tcan4550::request::TCAN455xRequest {
    pub fn set_sidfc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_SIDFC;
        let data: u32 = (MRAMCONFIG_NUMOFELEMENTS_SID << 16) | MRAM_OFFSETADDR_SID as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_xidfc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_XIDFC;
        let data: u32 = (MRAMCONFIG_NUMOFELEMENTS_XID << 16) | MRAM_OFFSETADDR_XID as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_rxf0c() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_RXF0C;
        let data: u32 = REG_BITS_MCAN_RXF0C_F0OM_OVERWRITE | (RXFIFO0_WM << 24) | (MRAMCONFIG_NUMOFELEMENTS_RXFIFO0 << 16) | MRAM_OFFSETADDR_RXFIFO0 as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_rxf1c() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_RXF1C;
        let data: u32 = REG_BITS_MCAN_RXF0C_F0OM_OVERWRITE | (RXFIFO1_WM << 24) | (MRAMCONFIG_NUMOFELEMENTS_RXFIFO1 << 16) | MRAM_OFFSETADDR_RXFIFO1 as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_rxbc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_RXBC;
        let data: u32 = MRAM_OFFSETADDR_RXBC as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_rxesc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_RXESC;
        let data: u32 = (RXBCDATASIZE.code << 8) | (RXFIFO1DATASIZE.code << 4) | (RXFIFO0DATASIZE.code);
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_txefc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_TXEFC;
        let data: u32 = (TXEFC_WM << 24) | (MRAMCONFIG_NUMOFELEMENTS_TXEFC << 16) | MRAM_OFFSETADDR_TXEFC as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_txbc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_TXBC;
        let data: u32 = (MRAMCONFIG_NUMOFELEMENTS_TXBC << 24) | (0 << 16) | MRAM_OFFSETADDR_TXBC as u32;
        Self::get_write_command(addr, vec![data])
    }

    pub fn set_txesc() -> Vec<u8> {
        let addr: u16 =  REG_MCAN_TXESC;
        let data: u32 = TXFIFODATASIZE.code;
        Self::get_write_command(addr, vec![data])
    }

    // Filter configuration
    pub fn set_sid(sidf_config: &[SIDFCONFIG]) -> Vec<u8> {
        let mut reg_data: Vec<u32> = Vec::new();
        for i in 0..MRAMCONFIG_NUMOFELEMENTS_SID as usize {
            reg_data.push((sidf_config[i].sft << 30) | (sidf_config[i].sfec << 27) | (sidf_config[i].sidf1 << 16) | sidf_config[i].sidf2);
        }

        let addr: u16 =  MRAM_STARTADDR_SID as u16;
        let data: Vec<u32> =  reg_data.to_vec();
        Self::get_write_command(addr, data)
    }

    pub fn set_xid(xidf_config: &[XIDFCONFIG]) -> Vec<u8> {
        let mut reg_data: Vec<u32> = Vec::new();
        for i in 0..MRAMCONFIG_NUMOFELEMENTS_XID as usize {
            reg_data.push((xidf_config[i].efec << 29) | xidf_config[i].eidf1);
            reg_data.push((xidf_config[i].eft << 30) | xidf_config[i].eidf2);
        }

        let addr: u16 =  MRAM_STARTADDR_XID as u16;

        Self::get_write_command(addr, reg_data)
    }

    pub fn get_txdata_start_addr(put_index: u16) -> u16 {
        MRAM_STARTADDR_TXBC + MRAMCONFIG_BYTESPERELEMENT_TXBC as u16 * put_index
    }
    
    pub fn get_rxdata_start_addr(ch: u16, get_index: u16) -> u16 {
      if ch == 0 {
        MRAM_STARTADDR_RXFIFO0 + MRAMCONFIG_BYTESPERELEMENT_RXFIFO0 as u16 * get_index
      }
      else {
        MRAM_STARTADDR_RXFIFO1 + MRAMCONFIG_BYTESPERELEMENT_RXFIFO1 as u16 * get_index
      }
    }
}

