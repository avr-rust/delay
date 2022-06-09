#![no_std]
#![crate_name = "avr_delay"]
#![feature(asm_experimental_arch)]

mod delay_impl;

/// This library is intended to provide a busy-wait delay
/// similar to the one provided by the arduino c++ utilities
/// If you need accurate time keeping you should consider a
/// hardware timer.

// This library does all of the busy-wait loop in rust.
// We pack as much of the looping as possible into asm!
// so that we can count cycles.

/// Internal function to implement a variable busy-wait loop. Even if count isn't
/// known at compile time, this function shouldn't have too much overhead.
/// # Arguments
/// * 'count' - a u32, the number of times to cycle the loop.
#[inline(always)]
pub fn delay(count: u32) {
    delay_impl::delay_count_32(count);
}

///delay for N milliseconds
/// # Arguments
/// * 'ms' - a u32, number of milliseconds to busy-wait. This should be known at
/// compile time, otherwise the delay may be much longer than specified.
#[inline(always)]
pub fn delay_ms(ms: u32) {
    const GCD: u32 = gcd(avr_config::CPU_FREQUENCY_HZ, 4_000);
    const NUMERATOR: u32 = avr_config::CPU_FREQUENCY_HZ / GCD;
    const DENOMINATOR: u32 = 4_000 / GCD;
    let ticks: u64 = (u64::from(ms) * u64::from(NUMERATOR)) / u64::from(DENOMINATOR);
    delay_impl::delay_count_48(ticks);
}

///delay for N microseconds
/// # Arguments
/// * 'ms' - a u32, number of microseconds to busy-wait. This should be known at
/// compile time, otherwise the delay may be much longer than specified.
#[inline(always)]
pub fn delay_us(us: u32) {
    const GCD: u32 = gcd(avr_config::CPU_FREQUENCY_HZ, 4_000_000);
    const NUMERATOR: u32 = avr_config::CPU_FREQUENCY_HZ / GCD;
    const DENOMINATOR: u32 = 4_000_000 / GCD;
    let ticks: u64 = (u64::from(us) * u64::from(NUMERATOR)) / u64::from(DENOMINATOR);
    delay_impl::delay_count_48(ticks);
}

const fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}