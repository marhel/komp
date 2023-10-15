When developing an embedded system, there's multiple ways to get your code onto the target device, and verifying that it behaves the way you want.

For the Raspberry Pi Pico microcontroller, we can build a new .uf2 firmware, flash it onto the device via USB and reset it, which starts your code. However this is a multiple step manual process, which includes rebooting the device with the BOOTSEL button held down. As the pico does not have a reset button, rebooting it typically involves cutting power by unplugging and reinserting the USB cable. Then you can only get limited debug output by writing text to the USB serial port. With rapid prototyping this gets tedious quickly, but we can do better.

We want to debug the device using "on chip debugging", for which we need a hardware debug probe which allows you to use a debugger on your host machine to set breakpoints, step through your code line by line, and inspect the values of variables. If you have a second Pico it can serve as a debug probe if you flash it with the [picoprobe](https://github.com/raspberrypi/picoprobe) firmware, or you can just get the pre-configured Raspberry Pi Debug Probe, which is more or less the same thing.
## Development environment: Linux
I happen to use a debian flavored Linux machine, so many of the example commands are unfortunately specific to this setup. This is not to say it isn't possible to get the probe running on Windows or macOS, I suggest looking more closely at the installation section of https://probe.rs/ if you aren't running Linux.
## Developing firmware for the Pico using Rust
I'll assume you already have [Rust](https://www.rust-lang.org/) and [rustup](https://rustup.rs/) installed. To develop for the Pico in particular you need to install the rust compilation target for ARM Cortex-M processor on the Pico.

	rustup target add thumbv6m-none-eabi

Now, if you want to use a debug probe, let's install some helpful tools;
### Install probe-rs
You can follow the instructions from https://probe.rs/, but in short they are (for a Linux host):

* install libusb and some other libraries that probe-rs needs:

	  sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev libssl-dev

* Then probe-rs can be built and installed using cargo

	  cargo install probe-rs --locked --features cli

## Running some sample code
After this (even if you don't have a probe) we should look at [[Blinking the on-board LED]]

References:
[The rp2040 project template](https://github.com/rp-rs/rp2040-project-template#requirements) 
https://slushee.dev/pico-tutorial/getting-started/

