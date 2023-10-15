# Get the picoprobe up and running
Getting the [picoprobe](https://github.com/raspberrypi/picoprobe) running, ready to debug your project involves a lot of steps and potentially some troubleshooting.
## Flash the picoprobe firmware
To flash the picoprobe firmware on a (second) Raspberry Pi Pico, follow these steps:

> Note: I do mean second, you cannot have both the picoprobe and your program running on the same Pico, they are one-trick ponys. You'll need one Pico to run the picoprobe firmware, to debug another Pico running your firmware.
 
* Download the picoprobe.uf2 firmware file from [the raspberrypi picoprobe releases](https://github.com/raspberrypi/picoprobe/releases/download/picoprobe-cmsis-v1.0.3/picoprobe.uf2) 
* While holding the BOOTSEL button on your (unpowered) Raspberry Pi Pico, connect your Pico to your computer using a USB cable. This boots the Pico in flash-drive mode.
* Release the BOOTSEL button when the RPI-RP2 drive appears on your computer. If a new drive does not appear, try a different USB cable as some are for charging only.
* Drag and drop the downloaded firmware file (.uf2) onto the RPI-RP2 drive.
* Wait a few seconds for the firmware to be flashed onto your Raspberry Pi Pico. It will reboot using the new firmware, and the RPI-RP2 drive should disappear.

> Note that building picoprobe from source is also possible but has way more steps and prerequisites. But according to [this picoprobe issue](https://github.com/raspberrypi/picoprobe/issues/41) it is however necessary if you intend to use the picoprobe firmware on the Pico W or WH versions. Then you need to build from source and pass the `--DPICO_BOARD=pico_w` flag to `cmake` (apparently, I haven't actually needed to try myself)
## Check if you can connect to the probe

Let's ask probe-rs to list the connected probes;

	$ probe-rs list
	The following debug probes were found:
	[0]: Picoprobe (CMSIS-DAP) (VID: 2e8a, PID: 000c, Serial: E6614103E75ABC30, CmsisDap)

If the output looks like above, congratulations, the probe was found, now look at [[Wiring the probe to the target]]

However, if this happened instead;

    $ probe-rs list
    No debug probes were found.

Then you need to do some troubleshooting, see [[Detecting the picoprobe]]