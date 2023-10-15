If we want to run without a debug probe we could look at [the rp-hal-boards repo](https://github.com/rp-rs/rp-hal-boards) which also has Rust support for the Raspberry Pi Pico. To run their blinky example without a probe, you can just do;

	$ git clone https://github.com/rp-rs/rp-hal-boards
	$ cd rp-hal-boards/boards/rp-pico
	# put your Pico in flash mode with BOOTSEL
	$ cargo run --release --example pico_blinky

This is because this repo is preconfigured to run elf2uf2-rs. 

> Maybe it is not obvious, but you can configure your own project to depend on rp-hal-boards, but still use whatever runner you like, so rp-hal-boards also works fine with a probe.
## Running with a probe
If you do have a debug probe, you need to first build, then call probe-rs manually;

	$ cargo build --release --example pico_blinky
	$ probe-rs run --chip rp2040 ../../target/thumbv6m-none-eabi/release/examples/pico_blinky
     Erasing sectors ✔ [00:00:00] [################################################################################################################################] 8.00 KiB/8.00 KiB @ 57.33 KiB/s (eta 0s )
    Programming pages   ✔ [00:00:00] [################################################################################################################################] 8.00 KiB/8.00 KiB @ 33.81 KiB/s (eta 0s )    Finished in 0.403s

As you can see from lack of output, this blinky does not have info logging set up, as opposed to when [[Blinking the LED with Embassy]]