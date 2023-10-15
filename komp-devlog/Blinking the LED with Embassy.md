We're going to use [the Embassy embedded development framework](https://embassy.dev/book/dev/getting_started.html) to make things easier for us going forward.

    $ git clone
    $ git clone https://github.com/embassy-rs/embassy.git
    $ cd embassy
    $ git submodule update --init

> Note that as of this writing, Embassy's "Getting started" documentation still recommends installing [probe-run, which is now deprecated in favor of probe-rs](https://ferrous-systems.com/blog/probe-run-deprecation/) so maybe don't bother installing it.

If you do have a probe setup, Embassy is already configured to use probe-rs to flash example code to your Pico, so you can just do a cargo run command to run their [blinky example](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/blinky.rs) for the Pico with;

> Maybe it is not obvious, but you can configure your own project to depend on Embassy, but still use whatever runner you like, so Embassy also works fine without a probe.

    $ cd examples/rp
	$ cargo run --bin blinky --release
	    Finished release [optimized + debuginfo] target(s) in 0.10s
	     Running `probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/blinky`
	     Erasing sectors ✔ [00:00:00] [####################################################################################################################################] 16.00 KiB/16.00 KiB @ 59.05 KiB/s (eta 0s )
	 Programming pages   ✔ [00:00:00] [####################################################################################################################################] 16.00 KiB/16.00 KiB @ 29.14 KiB/s (eta 0s )    Finished in 0.848s
	0.002388 INFO led on!
	└─ src/bin/blinky.rs:22
	1.002467 INFO led off!
	└─ src/bin/blinky.rs:26
	2.002490 INFO led on!
	└─ src/bin/blinky.rs:22
	3.002511 INFO led off!
	└─ src/bin/blinky.rs:26
	^C

> FYI: If you get compile errors here, it may be because you are using stable rust, and embassy still needs nightly rust (at the time of writing this, it does). Either switch to [[Blinking the LED with RP-HAL-boards]] or use rustup to change the active toolchain to nightly. Also note you will need the _thumbv6m-none-eabi_ target installed on the nightly toolchain as well.

This output shows that flashing the target Pico via the probe works, and the INFO lines shows that the UART serial output is forwarded by the probe, and hopefully the on-board LED blinks as well.

> Note that it _will not blink_ if you are using a Pico W or WH, as the on-board LED is wired differently (GP25 is wired to the on-board wifi/bluetooth module instead of the LED like on other Picos) and you should try [the wifi_blinky example](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs) instead.
## Running without a probe
If you wanted to run blinky without a picoprobe, you could. One way would be to use [elf2uf2-rs](https://github.com/jonil/elf2uf2-rs). Install it using `cargo install elf2uf2-rs --locked` and then;

* Build the blinky target using `cargo build --bin blinky --release`
* This builds an ELF-format binary for the `thumbv6m-none-eabi` compile target due to the settings in `embassy-rs/embassy/examples/rp/.cargo/config.toml`. However, we can only flash .uf2 images, which is why we installed `elf2uf2-rs` which knows how to convert ELF to UF2 format.
* Put your Pico in flash mode by holding the BOOTSEL button, connecting it to your host computer via USB, holding the button until the RPI-RP2 drive appears.
* Then run `elf2uf2-rs -d target/thumbv6m-none-eabi/release/blinky`
* The device will reset after the flashing is done, and start blinking the on-board LED.

> This can be simplified to just `cargo run` by setting elf2uf2-rs as the runner in `.cargo/config.toml` 

However, the UART serial output is not captured in this mode. There's an option (-s) for `elf2uf2-rs` to connect a virtual USB serial port (typically /dev/ttyACM0) that appears on the host when the new firmware is running, but this requires the firmware to be explicitly written to initiate USB serial communication, and there's no way to capture UART output without extra equipment (like a probe).