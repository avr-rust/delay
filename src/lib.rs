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

// This is probably not the right way to handle CPU speed.
// Arduino runs at 16MHZ
const MCU_SPEED: u32 = 16000000;

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - an i32, the number of times to cycle the loop.
pub fn delay(count: u32) {
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

///delay for N miliseconds
/// # Arguments
/// * 'ms' - an u32, number of milliseconds to busy-wait
pub fn delay_ms(ms: u32) {
    // microseconds
    let us = ms * 1000;
    delay_us(us);
}

///delay for N microseconds
/// # Arguments
/// * 'ms' - an u32, number of microseconds to busy-wait
pub fn delay_us(us: u32) {
    // picoseconds
    let ps = us * 1000;
    let ps_lp = 1000000000 / (MCU_SPEED / 4);
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

