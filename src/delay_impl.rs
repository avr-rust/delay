#[allow(unused_imports)]
use core::arch::asm;

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - an u32, the number of times to cycle the loop (4 clock cycles per loop).
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
/// * 'count' - an u64, the number of times to cycle the loop (4 clock cycles per loop). *The top 16 bits are ignored.*
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
/// * 'cycles' - an u16, the number of times to cycle the loop.
#[inline(always)]
#[allow(unused_variables, unused_mut, unused_assignments, dead_code)]
pub fn delay_loop_4_cycles(mut cycles: u16) {
    #[cfg(target_arch = "avr")]
    unsafe {
        asm!("1: sbiw {i}, 1",
            "brne 1b",
            i = inout(reg_iw) cycles => _,
        )
    }
    // Allow compilation even on non-avr targets, for testing purposes
}
