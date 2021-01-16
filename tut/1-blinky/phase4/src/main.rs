#![feature(asm)]
#![feature(global_asm)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;
use core::ptr::{write_volatile};

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe {
            asm!("nop" :::: "volatile");
        }
    }
}

unsafe fn kmain() -> ! {
    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    write_volatile(GPIO_FSEL1, 0b0100_0000_0000_0000_0000u32);
    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    loop {
        // write a 1 to 16th position of the set0 register to output
        write_volatile(GPIO_SET0, 0b1_0000_0000_0000_0000);
        spin_sleep_ms(200);

        // write a 1 to 16th position of the clr0 register to clear 
        write_volatile(GPIO_CLR0, 0b1_0000_0000_0000_0000);
        spin_sleep_ms(200);
    }
}
