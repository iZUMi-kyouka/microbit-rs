#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use panic_halt as _;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();

    const GPIO0_PINCNF13_ROW1_ADDR: *mut usize = 0x5000_0734 as *mut usize;
    const GPIO0_PINCNF4_COL1_ADDR: *mut usize = 0x5000_0710 as *mut usize;
    const DIR_OUTPUT_POS: usize = 0;
    const PINCNF_DRIVE_LED: usize = 1 << DIR_OUTPUT_POS;

    write_volatile(GPIO0_PINCNF13_ROW1_ADDR, PINCNF_DRIVE_LED);
    write_volatile(GPIO0_PINCNF4_COL1_ADDR, PINCNF_DRIVE_LED);

    const GPIO0_OUT_ADDR: *mut usize = 0x5000_0504 as *mut usize;
    // const GPIO_OUT_PINCNF13_ROW1_POS: usize = 13;
    // const GPIO_OUT_PINCNF4_COL1_POS: usize = 4;
    const GPIO_OUT_LED: usize = (1 << 13) | (1 << 4);
    write_volatile(GPIO0_OUT_ADDR, GPIO_OUT_LED);


    loop {
        // Turn LED1 on (PIN 13 high, PIN 4 low)
        nop();
    }
}