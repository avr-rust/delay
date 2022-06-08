#![feature(asm_experimental_arch)]

#![no_std]

#![crate_name = "avr_delay"]

#[allow(unused_imports)]
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

/// Internal function to implement a variable busy-wait loop. It is not recommended to use this function directly.
/// # Arguments
/// * 'count' - a u32, the number of times to cycle the loop.
#[inline(always)]
pub fn delay(count: u32) {
    delay_count_32(count);
}

///delay for N miliseconds
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
/// * 'ms' - an u32, number of microseconds to busy-wait
#[inline(always)]
pub fn delay_us(us: u32) {
    // picoseconds
    let ps = us * 1000;
    let ps_lp = 1000000000 / (avr_config::CPU_FREQUENCY_HZ / 4);
    let loops = (ps / ps_lp) as u32;
    delay(loops);
}

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - a u32, the number of times to cycle the loop (4 clock cycles per loop).
#[inline(always)]
pub fn delay_count_32(count: u32) {
    let mut outer_count: u16 = (count >> 16) as u16;
    let inner_count: u16 = count as u16;
    if inner_count != 0 {
        delay_loop_4_cycles(inner_count);
    }
    while outer_count != 0 {
        delay_loop_4_cycles(0);
        outer_count -= 1;
    } 
}

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - a u64, the number of times to cycle the loop (4 clock cycles per loop). *The top 16 bits are ignored.*
#[inline(always)]
pub fn delay_count_48(count: u64) {
    let mut outer_count: u32 = (count >> 16) as u32;
    let inner_count: u16 = count as u16;
    if inner_count != 0 {
        delay_loop_4_cycles(inner_count);
    }
    while outer_count != 0 {
        delay_loop_4_cycles(0);
        outer_count -= 1;
    } 
}

/// Internal function to implement a 16-bit busy-wait loop in assembly.
/// Delays for 4 cycles per iteration, not including setup overhead.
/// Up to 2^16 iterations (the value 2^16 would have to be passed as 0).
/// # Arguments
/// * 'cycles' - a u16, the number of times to cycle the loop.
#[inline(always)]
#[allow(unused_variables, unused_mut, unused_assignments, dead_code)]
fn delay_loop_4_cycles(mut cycles: u16) {
    #[cfg(target_arch = "avr")]
    unsafe {
        asm!("1: sbiw {i}, 1",
            "brne 1b",
            i = inout(reg_iw) cycles => _,
        )
    }
    // Allow compilation even on non-avr targets, for testing purposes
}

/// Internal function to implement an 8-bit busy-wait loop in assembly.
/// Delays for 3 cycles per iteration, not including setup overhead.
/// Up to 2^8 iterations (the value 2^8 would have to be passed as 0).
/// # Arguments
/// * 'cycles' - a u8, the number of times to cycle the loop.
#[inline(always)]
#[allow(unused_variables, unused_mut, unused_assignments, dead_code)]
fn delay_loop_3_cycles(mut cycles: u8) {
    #[cfg(target_arch = "avr")]
    unsafe {
        asm!("1: dec {i}",
            "brne 1b",
            i = inout(reg) cycles => _,
        )
    }
    // Allow compilation even on non-avr targets, for testing purposes
}