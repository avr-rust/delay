#![crate_name = "avr-delay"]
/// This library is intended to provide a busy-wait delay
/// similar to the one provided by the arduino c++ utilities
/// If you need accurate time keeping you should consider a
/// timer.

#![feature(asm)]

fn _delay(count: i32) {
    /// Internal function to implement a variable busy-wait loop.
    /// # Arguements
    /// * 'count' - an i32, the number of times to cycle the loop.
    for _ in 0..count {
        // needed to keep the loop from being optimized away
        unsafe {asm!("" :::: "volatile")}
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

