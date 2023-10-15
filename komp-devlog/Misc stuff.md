This page lists snippets and links that I haven't categorized properly yet.

possibly
	sudo udevadm control --reload-rules
(see https://embedded-trainings.ferrous-systems.com/installation.html?highlight=udev#linux-only-usb)

Detecting the picoprobe
https://embassy.dev/book/dev/getting_started.html

## Finding the target pico serial port
The picoprobe adds a virtual serial port to the host machine, which forwards what is sent from the target pico's uart0. This port can be found at /dev/ttyACMn , such as /dev/ttyACM0. 

To verify which one it is, you can query it as root using udevadm

    sudo udevadm info -n /dev/ttyACM0

or check /dev/serial/by-id for picoprobe;

    $ ls -l /dev/serial/by-id/
	total 0
	lrwxrwxrwx 1 root root 13 okt  8 23:20 usb-Raspberry_Pi_Picoprobe__CMSIS-DAP__E6614103E75ABC30-if01 -> ../../ttyACM0

Which shows us that the picoprobe serial port is indeed /dev/ttyACM0

## Looking at the serial output

minicom -b 115200 -o -D /dev/ttyACM0
> Note: To exit `minicom`, press CTRL-A X.

looking at the serial output via this only for "plain" serial comms, not data sent via the info!() macro, which you can only see when running via probe-rs.

The info!() macro is built using  [defmt](https://defmt.ferrous-systems.com/) encoded output, where `defmt` achieves high performance using deferred formatting and string compression, which means the minicom output will be unintelligible.

