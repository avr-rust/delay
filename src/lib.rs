#![feature(asm_experimental_arch)]
#![no_std]
#![crate_name = "avr_delay"]

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
/// * 'count' - an u32, the number of times to cycle the loop.
#[inline(always)]
pub fn delay(count: u32) {
    delay_impl::delay_count_32(count);
}

///delay for N milliseconds
/// # Arguments
/// * 'ms' - an u32, number of milliseconds to busy-wait. This should be known at
/// compile time, otherwise the delay may be much longer than specified.
#[inline(always)]
pub fn delay_ms(ms: u32) {
    let ticks: u64 = (u64::from(avr_config::CPU_FREQUENCY_HZ) * u64::from(ms)) / 4_000;
    delay_impl::delay_count_48(ticks);
}

///delay for N microseconds
/// # Arguments
/// * 'ms' - an u32, number of microseconds to busy-wait. This should be known at
/// compile time, otherwise the delay may be much longer than specified.
#[inline(always)]
pub fn delay_us(us: u32) {
    let ticks: u64 = (u64::from(avr_config::CPU_FREQUENCY_HZ) * u64::from(us)) / 4_000_000;
    delay_impl::delay_count_48(ticks);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
