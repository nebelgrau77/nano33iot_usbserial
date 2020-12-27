### arduino_nano33iot serial over USB

Code from a PyBadge/EdgeBadge example (which works) ported to Arduino Nano 33 IoT. 

Care must be taken when defining the buffer size, otherwise it won't work.
Same with .unwrap() of the serial.write() - won't work if unwrapped.

Uses the latest version of the crate, cloned from the official ATSAMD Rust repo.
