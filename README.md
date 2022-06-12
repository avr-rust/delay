# avr-delay


[![Crates.io](https://img.shields.io/crates/v/avr_delay.svg)](https://crates.io/crates/avr_delay)

[API Documentation](https://docs.rs/avr_delay/)

The intent of this library is to provide avr specific delay routines similar to the ones provided by the arduino library.

## `$AVR_CPU_FREQUENCY_HZ`

This crate uses the [`avr-config`](https://crates.io/crates/avr-config) crate for fetching the CPU frequency. As such, the `AVR_CPU_FREQUENCY_HZ` environment variable will need to be set when compiling your crate for AVR.

```bash
export AVR_CPU_FREQUENCY_HZ=16000000
cargo build -Z build-std=core --target avr-atmega328p.json --release
```

## API

```rust
delay_cycles<const CYCLES: u64>()
delay_us<const US: u64>()
delay_ms<const MS: u64>()
delay_sec<const SEC: u64>()
```

`delay_cycles` accepts 0 to 25_769_803_784 cycles (almost 18 minutes at 24Mhz).

The other functions convert time to cycles by using CPU_FREQUENCY_HZ.

```rust
delay_ms<42>(); // delay by 42ms (exactly 1_008_000 cycles at 24Mhz).
```

## Example

A simple example of how to use it follows.

Cargo.toml:

```toml
    [package]
    name = "dlyblink"
    version = "0.1.0"
    authors = ["John Jorgensen"]

    [dependencies]
    arduino = "0.1"
    avr_delay = { git = "https://github.com/avr-rust/delay" }
```

and your main.rs:

```rust
#![feature(asm, lang_items, unwind_attributes)]

#![no_std]
#![no_main]

extern crate arduino;
extern crate avr_delay;

use arduino::{DDRB, PORTB};
use core::ptr::write_volatile;
use avr_delay::delay_ms;

#[no_mangle]
pub extern fn main() {
    let mut out: u8 = 0x00;
    unsafe { write_volatile(DDRB, 0xff) }
    loop {
        out = out ^ 0xff;
        unsafe { write_volatile(PORTB, out) }
        delay_ms<1000000>();
    }
}

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }
    
    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) -> ! {
        loop { }
    }
}
```

No attempt is made to handle arithmetic overruns.


### Internal notes

#### Cargo publish

During `cargo publish` there could be
```text
error: ran out of registers during register allocation
```
With that _blocking issue_ is adding `--no-verify` considered
the _lesser evil_ as not releasing.
