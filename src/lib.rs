#![feature(asm)]

#![no_std]
#![no_main]

#![crate_name = "avr_delay"]

/// This library is intended to provide a busy-wait delay
/// similar to the one provided by the arduino c++ utilities
/// If you need accurate time keeping you should consider a
/// hardware timer.

// This library does all of the busy-wait loop in rust.
// The accuracy of time passed is highly dependent on the output
// of llvm.
//
// The arduino library takes the approach of packing a lot of the
// loop cycles into asm, where clocks can be counted, and the
// outer loops add less to the inaccuracy.

// This is probably not the right way to handle CPU speed.
// Arduino runs at 16MHZ
const MCU_SPEED: i32 = 16000000;

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - an i32, the number of times to cycle the loop.
fn _delay(count: i32) {
    for _ in 0..count {
        // needed to keep the loop from being optimized away
        unsafe {asm!("" :::: "volatile")}
    }
}

///delay for N miliseconds
/// # Arguments
/// * 'ms' - an i32, number of milliseconds to busy-wait
pub fn delay_ms(ms: i32) {
    // Ain't nothin' better than magic numbers!
    let dly_cnt = MCU_SPEED / 20000 * ms;
    _delay(dly_cnt);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

