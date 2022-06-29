use core::arch::asm;

/// Delayer::delay_impl() generates the inline assembly to delay by an exact amount of cycles.
///
/// The total number of cycles is computed as CYCLES * MUL / DIV.
/// With a maximum of 25_769_803_784 cycles.
///
/// Zero cycles does nothing. One cycle emits a `nop` instruction. 2 cycles one `rjump`. Above 5
/// cycles, we get into loops. With counters starting at 8 bits, and progressing through 16, 24,
/// and ultimately 32 bits. For a maximum of 11 instructions.
///
///
/// Two nightly features are required for this implementation:
/// #![feature(asm_experimental_arch)]
/// #![feature(asm_const)]
///
/// When the rustc `feature(generic_const_exprs)` is complete
/// (https://github.com/rust-lang/rust/issues/76560) it will become possible to do this directly:
/// ```
/// fn delay_ms<const SECS: u64>() {
///     Delayer::<{SECS * CPU_FREQUENCY_HZ / 1000}>::delay_impl();
/// }
/// ```
///
/// This is also why the code is structured in such a way. With everything as associated consts.
/// Because do support evaluation of expressions at compile time just fine contrary to const
/// generics. The implementation goes from generic consts, to associated consts on the Delayer
/// struct. And in turn those associated consts are fed to the `asm!` macro.
///
/// The rustc `feature(asm_const)` is also a work in progress
/// (https://github.com/rust-lang/rust/issues/93332). It appears to work well in the present code.
/// It also depends on the feature discussed in the next paragraph.
///
/// When `feature(inline_const)` (https://github.com/rust-lang/rust/issues/76001) is complete, all
/// the conditionals used in `delay_impl()` can be wrapped within `const {}` blocks. To ensure
/// beyond a shadow of a doubt that the whole function is fully linearised at compile time.
/// Nevertheless; thanks to constant propagation; this already happens implicitly.
///
/// The maximum number of cycles is 25_769_803_784 when `delay_cycles_u32()` iterates 2^32
/// times, `delay_2cycles()` is used twice, and `delay_1cycle()` once.
///
/// Every `delay_cycles_u*()` function has a minimum and maximum number of cycles it can consume.
///     The minimum is: (cycles per run).
///     The maximum is: (cycles per run) + (cycles per iteration) * (counter-1).
///     Note that a counter of zero iterates 2^bits time.
///
/// Example with `delay_cycles_u32()`.
///     Minimum: 9 cycles with 1 iteration.
///     Maximum: 9 + 6 * (2^32-1) == 25_769_803_779 cycles with 2^32 iterations.
///
/// Cycles 1..=5 are implemented by a combination of up to two `delay_2cycles()` and up to one
/// `delay_1cycle()`. Which gets us our maximum of 25_769_803_779 + 5 == 25_769_803_784.
///
/// Technically, beyond this value, the counters of various sizes will be combined until they are
/// all used up. This means the absolute limit is the sum of the maximum cycles of all counters
/// combined plus five:
///     (3+3*0xFF) + (5+4*0xFFFF) + (7+5*0xFF_FFFF) + (9+6*0xFFFF_FFFF) + 5 == 25_853_952_779.
/// But at this point, this is costing 23 instructions, for very little gain (~3.5s at 24Mhz).
/// Calling delay_cycles twice would be far more efficient.
pub struct Delayer<const CYCLES: u64, const MUL: u64, const DIV: u64>;

struct Cycles {
    counter_mask: u64,
    cycles_per_run: u64,
    cycles_per_iter: u64,
    max_cycles: u64,
}

struct Selection {
    selected: bool,
    counter: u64,
    remainder: u64,
}

const fn cycles(counter_mask: u64, cycles_per_run: u64, cycles_per_iter: u64) -> Cycles {
    Cycles {
        counter_mask,
        cycles_per_run,
        cycles_per_iter,
        max_cycles: cycles_per_run + cycles_per_iter * counter_mask,
    }
}

const fn select(info: Cycles, cycles: u64, above: u64) -> Selection {
    if !(cycles > above) {
        return Selection { selected: false, counter: 0, remainder: cycles };
    }
    let counter = (cycles - info.cycles_per_run) / info.cycles_per_iter + 1;
    let counter = if counter > info.counter_mask {
        info.counter_mask + 1
    } else {
        counter
    };
    Selection {
        selected: true,
        counter: if counter > info.counter_mask {
            0 // Counter wrap around.
        } else {
            counter
        },
        remainder: cycles - (info.cycles_per_run + info.cycles_per_iter * (counter - 1))
    }
}

impl<const CYCLES: u64, const MUL: u64, const DIV: u64> Delayer<CYCLES, MUL, DIV> {
    // Compute the intended number of cycles to delay, and panic if it is greater than
    // the maximum supported amount of cycles
    const TOTAL_CYCLES: u64 = {
        const MAX_SUPPORTED_CYCLES: u64 = 25_769_803_784;
        // Multiply first to avoid precision loss, and expand to u128 to avoid overflow
        let result: u128 = (CYCLES as u128) * (MUL as u128) / (DIV as u128);
        assert!(result <= (MAX_SUPPORTED_CYCLES as u128), "Error: Tried to delay for too many cycles. The maximum supported delay is 25_769_803_784 cycles");
        result as u64
    };

    // counter mask, cycles per run, cycles per iteration. | cost + worst case remainder cost
    const U32_INFO: Cycles = cycles(0xFFFF_FFFF, 9, 6); // 8 + 3
    const U24_INFO: Cycles = cycles(  0xFF_FFFF, 7, 5); // 6 + 2
    const U16_INFO: Cycles = cycles(     0xFFFF, 5, 4); // 4 + 2
    const  U8_INFO: Cycles = cycles(       0xFF, 3, 3); // 3 + 1

    // The selection process stops at the smallest counter size that can handle the number of
    // cycles to consume with a remainder of up to 5 cycles. This will not always produce the
    // smallest possible number of instructions. In some cases, the cost of U16+U8 might be one
    // instruction lower than that of the U24. This is because the U16+U8 would have no remainder
    // contrary to the U24. Many combinations of the various counter sizes are possible, dividing
    // the number of cycles more or less evenly. Implementing this without
    // `feature(generic_const_exprs) seems daunting. It would require to compute the various
    // combinations and compare the cost. Note that gcc-avr intrinsics delay_cycles
    // doesn't bother to optimize this if this can be of any consolation.
    const U32: Selection = select(Self::U32_INFO, Self::TOTAL_CYCLES,  Self::U24_INFO.max_cycles + 4);
    const U24: Selection = select(Self::U24_INFO, Self::U32.remainder, Self::U16_INFO.max_cycles + 5);
    const U16: Selection = select(Self::U16_INFO, Self::U24.remainder, Self::U8_INFO.max_cycles + 4);
    const U8 : Selection = select(Self::U8_INFO,  Self::U16.remainder, 5);
    // The extras +4, +5, and +4 cycles take into account that even though the number of cycles is
    // beyond the capacity of the counter, the overflow can be served by the 1.=5 cycles
    // implementation. In those instances, it so happens that the counter of the next size up would
    // take more instructions because it also requires a remainder.

    // The counters leave up to 5 cycles as a remainder. They are consumed with up to two `rjump`
    // and a `nop`.
    // 5 cycles => 3 instructions.
    // 4 cycles => 2 instructions.
    // 3 cycles => 2 instructions.
    // 2 cycles => 1 instruction.
    // 1 cycle  => 1 instruction.

    /// 8 instructions.
    /// 9 cycles per run.
    /// 6 cycles per iteration.
    #[inline(always)]
    fn delay_cycles_u32() {
        unsafe {
            asm!(
                "ldi {r0:l}, {b0}",
                "ldi {r0:h}, {b1}",
                "ldi {r2}, {b2}",
                "ldi {r3}, {b3}",
                "1:",
                "sbiw {r0}, 1",
                "sbci {r2}, 0",
                "sbci {r3}, 0",
                "brne 1b",
                r0 = out(reg_iw) _,
                r2 = out(reg_upper) _,
                r3 = out(reg_upper) _,
                b0 = const (Self::U32.counter >> 0) as u8,
                b1 = const (Self::U32.counter >> 8) as u8,
                b2 = const (Self::U32.counter >> 16) as u8,
                b3 = const (Self::U32.counter >> 24) as u8,
                options(nomem, nostack),
            )
        }
    }

    /// 6 instructions.
    /// 7 cycles per run.
    /// 5 cycles per iteration.
    #[inline(always)]
    fn delay_cycles_u24() {
        // Some way to static assert that COUNTER < 2^24 would be nice.
        unsafe {
            asm!(
                "ldi {r0:l}, {b0}",
                "ldi {r0:h}, {b1}",
                "ldi {r2}, {b2}",
                "1:",
                "sbiw {r0}, 1",
                "sbci {r2}, 0",
                "brne 1b",
                r0 = out(reg_iw) _,
                r2 = out(reg_upper) _,
                b0 = const (Self::U24.counter >> 0) as u8,
                b1 = const (Self::U24.counter >> 8) as u8,
                b2 = const (Self::U24.counter >> 16) as u8,
                options(nomem, nostack),
            )
        }
    }

    /// 4 instructions.
    /// 5 cycles per run.
    /// 4 cycles per iteration.
    #[inline(always)]
    fn delay_cycles_u16() {
        unsafe {
            asm!(
                "ldi {r0:l}, {b0}",
                "ldi {r0:h}, {b1}",
                "1:",
                "sbiw {r0}, 1",
                "brne 1b",
                r0 = out(reg_iw) _,
                b0 = const (Self::U16.counter >> 0) as u8,
                b1 = const (Self::U16.counter >> 8) as u8,
                options(nomem, nostack),
            )
        }
    }

    /// 3 instructions.
    /// 3 cycles per run.
    /// 3 cycles per iteration.
    #[inline(always)]
    fn delay_cycles_u8() {
        unsafe {
            asm!(
                "ldi {r0}, {b0}",
                "1:",
                "dec {r0}",
                "brne 1b",
                r0 = out(reg_upper) _,
                b0 = const Self::U8.counter,
                options(nomem, nostack),
                // The carry flag is not touched by `dec`.
                // That's the difference between `dec` and `sub 1`.
                // Is it possible to tell `asm!` that the carry is untouched?
                // Something like `preserves_carry_flag`.
                // The compiler wouldn't have to save the carry flag when delay_cycles_u8 is used
                // within an outer loop using multiple-precision computations.
            )
        }
    }

    /// 1 instruction.
    /// 2 cycles per run.
    #[inline(always)]
    fn delay_2cycles() {
        unsafe { asm!("rjmp .", options(nomem, nostack, preserves_flags),) }
    }

    /// 1 instruction.
    /// 1 cycle per run.
    #[inline(always)]
    fn delay_1cycle() {
        unsafe { asm!("nop", options(nomem, nostack, preserves_flags),) }
    }

    #[inline(always)]
    pub fn delay_impl() {
        // Cycles 83_886_083 + 4 .. 25_769_803_779 (9+6*0xFFFF_FFFF) + 5
        if Self::U32.selected {
            Self::delay_cycles_u32();
        }

        // Cycles 262_146 + 5 ..= 83_886_082 (7+5*0xFF_FFFF) + 4
        if Self::U24.selected {
            Self::delay_cycles_u24();
        }

        // Cycles 769 + 4 ..= 262_145 (5+4*0xFFFF) + 5
        if Self::U16.selected {
            Self::delay_cycles_u16();
        }

        // Cycles 6 ..= 768 (3+3*0xFF) + 4
        if Self::U8.selected {
            Self::delay_cycles_u8();
        }

        // Remaining cycles 1..=5.

        if Self::U8.remainder >= 4 {
            Self::delay_2cycles();
        }

        if Self::U8.remainder >= 2 {
            Self::delay_2cycles();
        }

        if Self::U8.remainder % 2 == 1 {
            Self::delay_1cycle();
        }
    }
}
