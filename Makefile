# Makefile useful during development.
# `make asm` compiles examples/codegen.rs to assembly.
# The output can then be reviewed for correctness.

NV?=nightly
override FLAGS += -Z build-std=core -Z build-std-features=panic_immediate_abort --release
MCU?=avr-atmega328p
AVR_CPU_FREQUENCY_HZ?=16000000
export AVR_CPU_FREQUENCY_HZ

build:
	cargo +$(NV) build $(FLAGS) --target=$(MCU).json -v
	cargo +$(NV) build --example codegen $(FLAGS) --target=$(MCU).json -v

asm:
	cargo +$(NV) rustc --example codegen $(FLAGS) --target=$(MCU).json -- --emit asm
	@echo target/$(MCU)/release/examples/*.s

asmclean:
	-rm target/*/release/deps/*.s
