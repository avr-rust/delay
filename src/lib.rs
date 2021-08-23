#![feature(llvm_asm)]

#![no_std]

#![crate_name = "avr_delay"]

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
/// * 'count' - a u64, the number of times to cycle the loop.
pub fn delay(count: u64) {
    // Our asm busy-wait takes a 16 bit word as an argument,
    // so the max number of loops is 2^16
    let outer_count = count / 65536;
    let last_count = ((count % 65536)+1) as u16;
    for _ in 0..outer_count {
        // Each loop through should be 4 cycles.
        unsafe {llvm_asm!("1: sbiw $0,1
                      brne 1b"
                     :
                     : "w" (0)
                     :
                     :)}
    }
    unsafe {llvm_asm!("1: sbiw $0,1
                      brne 1b"
                 :
                 : "w" (last_count)
                 :
                 :)}
}

///delay for N milliseconds
/// # Arguments
/// * 'ms' - a u64, number of milliseconds to busy-wait
pub fn delay_ms(ms: u64) {
    // microseconds
    let us = ms * 1000;
    delay_us(us);
}

///delay for N microseconds
/// # Arguments
/// * 'us' - a u64, number of microseconds to busy-wait
pub fn delay_us(us: u64) {
    let us_in_loop = (avr_config::CPU_FREQUENCY_HZ / 1000000 / 4) as u64;
    let loops = us * us_in_loop;
    delay(loops);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

