#![crate_name = "avr_delay"]
#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

mod delay_cycles;

use delay_cycles::Delayer;

/// Delay by the exact number of CYCLES.
/// The number of instructions generated goes up to 11. The higher the number of cycles, the higher
/// number of instructions, in a staircase effect.
/// Accepts 0 to 25_769_803_784 cycles (almost 18 minutes at 24Mhz).
#[inline(always)]
pub fn delay_cycles<const CYCLES: u64>() {
    Delayer::<CYCLES, 1, 1>::delay_impl()
}

/// Maximum value is (25_769_803_784 * 1_000_000 / CPU_FREQUENCY_HZ).
/// Almost 18 minutes at 24Mhz.
#[inline(always)]
pub fn delay_us<const US: u64>() {
    Delayer::<US, {avr_config::CPU_FREQUENCY_HZ as u64}, 1_000_000>::delay_impl()
}

/// Maximum value is (25_769_803_784 * 1_000 / CPU_FREQUENCY_HZ).
/// Almost 18 minutes at 24Mhz.
#[inline(always)]
pub fn delay_ms<const MS: u64>() {
    Delayer::<MS, {avr_config::CPU_FREQUENCY_HZ as u64}, 1_000>::delay_impl()
}

/// Maximum value is (25_769_803_784 * 1 / CPU_FREQUENCY_HZ).
/// Almost 18 minutes at 24Mhz.
#[inline(always)]
pub fn delay_sec<const SEC: u64>() {
    Delayer::<SEC, {avr_config::CPU_FREQUENCY_HZ as u64}, 1>::delay_impl()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
