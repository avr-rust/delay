# avr-delay


[![Crates.io](https://img.shields.io/crates/v/avr_delay.svg)](https://crates.io/crates/avr_delay)

[API Documentation](https://docs.rs/avr_delay/)

The intent of this library is to provide avr specific delay routines similar to the ones provided by the arduino library. The public functions are:

## `$AVR_CPU_FREQUENCY_HZ`

This crate uses the [`avr-config`](https://crates.io/crates/avr-config) crate for fetching the CPU frequency. As such, the `AVR_CPU_FREQUENCY_HZ` environment variable will need to be set when compiling your crate for AVR.

Example:

```bash
export AVR_CPU_FREQUENCY_HZ=16000000
cargo build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
```

```rust
delay(count: u32)
```

is a raw delay loop. Each loop is 4 cycles. The asm section can loop 65536 times. Initial overhead is about 13 cycles. Each outer loop has an overhead of about 11 cycles.

```rust
delay_us(us: u32)
```

delay _us_ microseconds

```rust
delay_ms(ms: u32)
```

delay _ms_ milliseconds

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
use avr_delay::{delay, delay_ms, delay_us};

#[no_mangle]
pub extern fn main() {
    let mut out: u8 = 0x00;
    unsafe { write_volatile(DDRB, 0xff) }
    loop {
        out = out ^ 0xff;
        unsafe { write_volatile(PORTB, out) }
        delay_ms(1000000);
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
