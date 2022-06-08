#![feature(asm_experimental_arch)]

#![no_std]

#![crate_name = "avr_delay"]

use core::arch::asm;

/// This library is intended to provide a busy-wait delay
/// similar to the one provided by the arduino c++ utilities
/// If you need accurate time keeping you should consider a
/// hardware timer.

// This library does all of the busy-wait loop in rust.
// We pack as much of the looping as possible into asm!
// so that we can count cycles.
//
// Ignoring the overhead, which may be significant:
// An arduino runs at 16MHZ. Each asm loop is 4 cycles.
// so each loop is 0.25 us.
//
// the overhead of delay() seems to be about 13 cycles
// initially, and then 11 cycles per outer loop. We ignore
// all that.

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - an i32, the number of times to cycle the loop.
#[inline(always)]
pub fn delay(count: u32) {
    // Our asm busy-wait takes a 16 bit word as an argument,
    // so the max number of loops is 2^16
    let outer_count = count / 65536;
    let last_count = ((count % 65536)+1) as u16;
    for _ in 0..outer_count {
        // Each loop through should be 4 cycles.
        let zero = 0u16;
        unsafe {
            asm!("1: sbiw {i}, 1",
                 "brne 1b",
                 i = inout(reg_iw) zero => _,
            )
        }
    }
    unsafe {
        asm!("1: sbiw {i}, 1",
             "brne 1b",
             i = inout(reg_iw) last_count => _,
        )
    }
}

///delay for N milliseconds
/// # Arguments
/// * 'ms' - an u32, number of milliseconds to busy-wait
#[inline(always)]
pub fn delay_ms(ms: u32) {
    // microseconds
    let us = ms * 1000;
    delay_us(us);
}

///delay for N microseconds
/// # Arguments
/// * 'us' - an u32, number of microseconds to busy-wait
#[inline(always)]
pub fn delay_us(us: u32) {
    // picoseconds
    let ps = us * 1000;
    let ps_lp = 1000000000 / (avr_config::CPU_FREQUENCY_HZ / 4);
    let loops = (ps / ps_lp) as u32;
    delay(loops);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

