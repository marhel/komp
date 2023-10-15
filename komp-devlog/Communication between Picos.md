I've set up two projects, **player** and **driver**. The player listens for commands over a serial connection (uart) and the driver sends such commands.
## I need four Picos, please!
As I want to use one Pico to verify the behavior of another, I have two target Picos. And since it is so much easier to develop using debug probes, I need an additional two Picos to run as debug probes.

My setup is therefore the following;

* I have a USB cable connecting a Pico H flashed as a debug probe [[Wiring the probe to the target|wired to]] a target Pico on the Pimoroni Pico Explorer board running my 'driver' firmware.
* I have another USB cable connecting a Pico flashed as a debug probe [[Wiring the probe to the target|wired to]] a target Pico W running my 'player' firmware.
* The 'driver' Pico is connected to the 'player' Pico in the following way;
	* driver UART1 RX (GP5) -> player UART1 TX (GP4)
	* driver UART1 TX (GP4) -> player UART1 RX (GP5)
## Panicking
If we only connect a single probe, and try to run the player, we get a huge error message;

    player$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/player --probe '2e8a:000c:E66118604B235F26'`
     Erasing sectors ✔ [00:00:00] [###############################################################################################################################] 64.00 KiB/64.00 KiB @ 65.11 KiB/s (eta 0s )
    Programming pages   ✔ [00:00:01] [###############################################################################################################################] 64.00 KiB/64.00 KiB @ 32.64 KiB/s (eta 0s )    Finished in 2.973s
	 0.008393 INFO Player listening...
	└─ src/bin/player.rs:37
	0.008808 ERROR panicked at src/bin/player.rs:41:33:
	called `Result::unwrap()` on an `Err` value: Break
	└─ /home/martin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:104
	DEBUG probe_rs::debug::unit_info: Found DIE, now checking for inlined functions: name=None
	DEBUG probe_rs::debug::unit_info: Found DIE, now checking for inlined functions: name=None
	DEBUG probe_rs::debug::unit_info: Found DIE, now checking for inlined functions: name=None
	DEBUG probe_rs::debug::unit_info: Found DIE, now checking for inlined functions: name=None
	DEBUG probe_rs::debug::unit_info: Found DIE, now checking for inlined functions: name=None
	DEBUG probe_rs::debug::unit_info: Found DIE, now checking for inlined functions: name=None
	Frame 0: HardFault @ 0x1000cf60
	Frame 1: __udf @ 0x100030e8 inline
	       /home/martin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cortex-m-0.7.7/src/../asm/inline.rs:181:5
	Frame 2: udf @ 0x00000000100030e8
	       /home/martin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cortex-m-0.7.7/src/asm.rs:43:5
	Frame 3: hard_fault @ 0x10003150
	       /home/martin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:86:5
	Frame 4: panic @ 0x10003142
	       /home/martin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:54:9
	Frame 5: panic_fmt @ 0x1000b946
	       /rustc/8ce4540bd6fe7d58d4bc05f1b137d61937d3cf72/library/core/src/panicking.rs:72:14
	Frame 6: unwrap_failed @ 0x1000b9f0
	       /rustc/8ce4540bd6fe7d58d4bc05f1b137d61937d3cf72/library/core/src/result.rs:1652:5
	Frame 7: <unknown function @ 0x10000bee> @ 0x10000bee
	       /rustc/8ce4540bd6fe7d58d4bc05f1b137d61937d3cf72/library/core/src/result.rs:1077:23
	Frame 8: {async_fn#0} @ 0x100002f0
	       /home/martin/github/marhel/komp/komp-emb/player/src/bin/player.rs:42:9
	Frame 9: <unknown function @ 0x100027de> @ 0x100027de
	       /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-executor/src/raw/mod.rs:161:15
	Frame 10: {closure#0} @ 0x1000a93a
	       /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-executor/src/raw/mod.rs:411:17
	Frame 11: <unknown function @ 0x1000a3a6> @ 0x1000a3a6
	       /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-executor/src/raw/run_queue.rs:85:13
	Frame 12: <unknown function @ 0x1000a8a4> @ 0x1000a8a4
	       /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-executor/src/raw/mod.rs:391:13
	Frame 13: <unknown function @ 0x1000a994> @ 0x1000a994
	       /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-executor/src/raw/mod.rs:533:6
	Frame 14: <unknown function @ 0x100005ca> @ 0x100005ca
	       /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-executor/src/arch/cortex_m.rs:107:21
	Frame 15: __cortex_m_rt_main @ 0x10001600
	       /home/martin/github/marhel/komp/komp-emb/player/src/bin/player.rs:26:1
	Frame 16: __cortex_m_rt_main_trampoline @ 0x100015e4
	       /home/martin/github/marhel/komp/komp-emb/player/src/bin/player.rs:26:1
	Frame 17: <unknown function @ 0x100001e6> @ 0x100001e6
	Frame 18: <unknown function @ 0x100001e6> @ 0x100001e6

The relevant part is in the beginning;

	 0.008393 INFO Player listening...
	└─ src/bin/player.rs:37
	0.008808 ERROR panicked at src/bin/player.rs:41:33:
	called `Result::unwrap()` on an `Err` value: Break
	└─ /home/martin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:104

This is actually good, because we can see that when running via the probe, we are notified when the program fails on-device, pointing to the location that panicked, in this case src/bin/player.rs line 41 seems to have called `.unwrap()` on an Err::Break value.

Handing the error makes the player more robust.
## Now were talking

	driver$ cargo run
	Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/driver --probe '2e8a:000c:E6614103E75ABC30'`
     Erasing sectors ✔ [00:00:01] [###############################################################################################################################] 72.00 KiB/72.00 KiB @ 61.96 KiB/s (eta 0s )
	 Programming pages   ✔ [00:00:02] [###############################################################################################################################] 72.00 KiB/72.00 KiB @ 33.14 KiB/s (eta 0s )    Finished in 3.373s
	0.008469 INFO Driver sending...
	└─ src/bin/driver.rs:42
	0.008688 WARN Framing error
	└─ /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-rp/src/fmt.rs:156
	0.008802 WARN Break error
	└─ /home/martin/.cargo/git/checkouts/embassy-9312dcb0ed774b29/fc8f96f/embassy-rp/src/fmt.rs:156
	0.009312 INFO TX [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]
	└─ src/bin/driver.rs:50
	1.010965 INFO TX [31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61]
	└─ src/bin/driver.rs:50
	2.012496 INFO TX [62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92]
	└─ src/bin/driver.rs:50
	3.013910 INFO TX [93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123]
	└─ src/bin/driver.rs:50


	player$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/player --probe '2e8a:000c:E66118604B235F26'`
     Erasing sectors ✔ [00:00:00] [###############################################################################################################################] 64.00 KiB/64.00 KiB @ 65.98 KiB/s (eta 0s )
	 Programming pages   ✔ [00:00:02] [###############################################################################################################################] 64.00 KiB/64.00 KiB @ 31.70 KiB/s (eta 0s )    Finished in 3.027s
	0.008335 INFO Player listening...
	└─ src/bin/player.rs:37
	0.826827 INFO RX [209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224]
	└─ src/bin/player.rs:42
	1.826575 INFO RX [225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240]
	└─ src/bin/player.rs:42
	1.827926 INFO RX [241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 0]
	└─ src/bin/player.rs:42
	2.827957 INFO RX [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
	└─ src/bin/player.rs:42
	2.829345 INFO RX [17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]
	└─ src/bin/player.rs:42
	3.829333 INFO RX [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48]
	└─ src/bin/player.rs:42
	3.830730 INFO RX [49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64]
	└─ src/bin/player.rs:42
	4.830817 INFO RX [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80]
	└─ src/bin/player.rs:42
	4.832218 INFO RX [81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96]
	└─ src/bin/player.rs:42
	5.832296 INFO RX [97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112]
	└─ src/bin/player.rs:42
	5.833689 INFO RX [113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128]
	└─ src/bin/player.rs:42
	6.833731 INFO RX [129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144]
	└─ src/bin/player.rs:42
	6.835125 INFO RX [145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160]
	└─ src/bin/player.rs:42
	7.835204 INFO RX [161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176]
	└─ src/bin/player.rs:42
	7.836595 INFO RX [177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192]
	└─ src/bin/player.rs:42
	8.836670 INFO RX [193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208]
	└─ src/bin/player.rs:42
	8.838063 INFO RX [209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224]
	└─ src/bin/player.rs:42
	9.838157 INFO RX [225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240]
	└─ src/bin/player.rs:42
	9.839542 INFO RX [241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 0]
	└─ src/bin/player.rs:42
	10.839595 INFO RX [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
	└─ src/bin/player.rs:42
	10.840986 INFO RX [17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]
