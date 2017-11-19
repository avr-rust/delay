# avr-delay

The intent of this library is to provide avr specific delay routines similar to the ones provided by the arduino library. 

A simple example of how to use it follows.

Cargo.toml:

    [package]
    name = "dlyblink"
    version = "0.1.0"
    authors = ["John Jorgensen <john@2gn.com>"]

    [dependencies]
    arduino = "0.1"
    avr_delay = { git = "https://github.com/pusherofbrooms/avr-delay" }

and your main.rs:

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

