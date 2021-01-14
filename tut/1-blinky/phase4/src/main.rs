#![feature(asm)]
#![feature(global_asm)]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;
use core::ptr::write_volatile;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

unsafe fn kmain() -> ! {
    // use std::ptr::write_volatile;
    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    let val = 0b001u32;
    // val << 17;
    write_volatile(GPIO_FSEL1, val << 17);


    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    loop {
        write_volatile(GPIO_SET0, 0b10000u32); // 16 as binary 
        spin_sleep_ms(1000);
        write_volatile(GPIO_CLR0, 0b10000u32); // 16 as binary
    }
}
