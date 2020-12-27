### arduino_nano33iot serial over USB

Code from a PyBadge/EdgeBadge example (which works) ported to Arduino Nano 33 IoT. 

Outputs values in EdgeImpulse data forwarder format, so it's ready for some sensor to be attached (ideally the LSM6DS3 from the Nano33 IoT board, but there's no crate yet).

NOTES:

* Care must be taken when defining the buffer size, otherwise it won't work. Same with .unwrap() of the serial.write() - won't work if unwrapped.
* Uses the latest version of the crate, cloned from the official ATSAMD Rust repo.
