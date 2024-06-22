use crate::tcan4550::register::*;

const WAKE_CONFIG: u32 = 3;      // Wake pin can be triggered by either edge (default)
const WD_TIMER: u32 = 0;         // Watchdog timer, 0: 60 ms, 1: 600 ms, 2: 3 s, 3: 6s
const CLK_REF: u32 = 1;          // Input crystal, 1: 40 MHz crystal, 0: 20 MHz
const GPO2_CONFIG: u32 = 0;      // GPO2, 0: No Action, 1: MCAN_INT 0 interrupt (Active low), 2: Watchdog output, 3: Mirrors nINT pin (Active low)
const TESTMODE_EN: u32 = 0;      // Test mode
const NWKRQ_VOLTAGE: u32 = 0;    // Set nWKRQ to 0: Internal voltage rail, 1: VIO voltage rail
const WD_BIT_RESET: u32 = 0;     // Don't reset the watchdog
const WD_ACTION: u32 = 0;        // Watchdog set an interrupt (default)
const GPIO1_CONFIG: u32 = 0;     // GPIO set as 0: GPO (Default), 2: GPI
const FAIL_SAFE_EN: u32 = 0;     // Failsafe disabled (default)
const GPIO1_GPO_CONFIG: u32 = 1; // GPO1, 0: SPI fault Interrupt (Active low), 1: MCAN_INT 1 (Active low), 2: UVO or TSD (Active low), 3: Reserved
const INH_DIS: u32 = 0;          // INH enabled (default)
const NWKRQ_CONFIG: u32 = 0;     // 0: Mirrors INH function, 1: Wake request interrupt
const MODE_SEL: u32 = 1;         // MODE_SEL, 0: sleep, 1: standby, 2: normal, 3: reserved
const WD_EN: u32 = 0;            // Watchdog disabled
const DEVICE_RESET: u32 = 0;     // Action at software reset, 0: Current configuration, 1: Device resets to default
const SWE_DIS: u32 = 0;          // Keep Sleep Wake Error Enabled (it's a disable bit, not an enable)
const TESTMODE_CONFIG: u32 = 0;  // Test mode, 0: PHY test, 1: CAN controller test

impl crate::tcan4550::request::TCAN455xRequest {
    pub fn set_device_modes_and_pins() -> Vec<u8> {
    let addr: u16 = REG_DEV_MODES_AND_PINS;
    let data: u32 = (WAKE_CONFIG << 30)
        | (WD_TIMER << 28)
        | (CLK_REF << 27)
        | (GPO2_CONFIG << 22)
        | (TESTMODE_EN << 21)
        | (NWKRQ_VOLTAGE << 19)
        | (WD_BIT_RESET << 18)
        | (WD_ACTION << 16)
        | (GPIO1_CONFIG << 14)
        | (FAIL_SAFE_EN << 13)
        | (GPIO1_GPO_CONFIG << 10)
        | (INH_DIS << 9)
        | (NWKRQ_CONFIG << 8)
        | (MODE_SEL << 6)
        | (WD_EN << 3)
        | (DEVICE_RESET << 2)
        | (SWE_DIS << 1)
        | (TESTMODE_CONFIG << 0);

        Self::get_write_command(addr, vec![data])
    }
}








