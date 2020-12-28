### Arduino Nano33IoT - print to serial over USB

Code from a PyBadge/EdgeBadge example ported to Arduino Nano 33 IoT. 

Outputs values in EdgeImpulse data forwarder format, ready for some sensor to be attached (ideally the LSM6DS3 from the Nano33 IoT board, but there's no crate yet).

NOTES:

* Care must be taken when defining the buffer size, otherwise it won't work. Same with .unwrap() of the serial.write() - won't work if unwrapped.
* Uses the latest version of the crate, cloned from the official ATSAMD Rust repo.

TO DO: 

* Modify it to work with interrupts instead of delays (RTIC, etc.) for precise timing.
