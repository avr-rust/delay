#![no_std]
#![no_main]

extern crate avr_std_stub;

#[no_mangle]
fn main() {
    avr_delay::delay_ms(4500);
}
