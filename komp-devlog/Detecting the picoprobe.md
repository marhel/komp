If `probe-rs list` says `No debug probes were found` then we need to do some troubleshooting. First do `lsusb` to see if the picoprobe is listed there.

	$ lsusb
	...
	Bus 001 Device 007: ID 2e8a:000c Raspberry Pi Picoprobe (CMSIS-DAP)
	...

If you instead see "Raspberry Pi RP2 Boot" then that is the pico flash-drive ready for being flashed, so if you already have flashed the picoprobe firmware, you need to boot that by unplugging and plugging the device from USB. If you have not yet flashed the picoprobe firmware, see [[Flashing the picoprobe]]

Then do ls -l /dev/bus/usb/{bus}/{device}, so in my case, bus 1 and device 7 I do:

    $ ls -l /dev/bus/usb/001/007
    crw-rw-r-- 1 root root 189, 6 okt  8 12:59 /dev/bus/usb/001/007

These permissions prevent anyone but root to use the device, which is not what we want.

## Allow access to the picoprobe
You then need to add a udev-rule to allow access to the picoprobe for authenticated users. Add a file named 50-pico.rules to `/etc/udev/rules.d/` with the contents;

    ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="000c", TAG+="uaccess"

> * Writing to /etc/udev/rules.d folder needs superuser permissions
> * Your idVendor and idProduct values need to match the ID that you saw in the output from `lsusb`
> * In Linux udev, `TAG+="uaccess"` is a rule that adds the `uaccess` tag to a device. This tag is used by `systemd` to dynamically grant access to devices to authenticated users.

I happen to like the nano editor, so I did `sudo nano /etc/udev/rules.d/50-pico.rules`

Then just unplug and plug the probe back into your computer,  and check the permissions of the probe again;

> Note that your device will have a different device number once reconnected.

	$ lsusb
	...
	Bus 001 Device 008: ID 2e8a:000c Raspberry Pi Picoprobe (CMSIS-DAP)
	...
    $ ls -l /dev/bus/usb/001/008
    crw-rw-r--+ 1 root root 189, 6 okt  8 12:59 /dev/bus/usb/001/008
    
Note the added `+`. Now try probe-rs again.

	$ probe-rs list
	The following debug probes were found:
	[0]: Picoprobe (CMSIS-DAP) (VID: 2e8a, PID: 000c, Serial: E6614103E75ABC30, CmsisDap)

Success! If not, hopefully there's something in [the probe-rs troubleshooting section](https://probe.rs/docs/knowledge-base/troubleshooting/) that will help you.

After getting the probe detected, you should look at [[Wiring the probe to the target]]