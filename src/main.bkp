//! this works, must be built with --features="usb"
//! 
//! care must be taken to get the right size of the buffer
//! 

#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
extern crate cortex_m;
extern crate panic_halt as _;
extern crate usb_device;
extern crate usbd_serial;

use hal::{clock::GenericClockController,
        entry,
        pac::{CorePeripherals, Peripherals},               
        prelude::*,
        delay::Delay,
        time::KiloHertz,
        };

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};


use core::fmt;

use arrayvec::ArrayString;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );    
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);
    
    let mut delay = Delay::new(core.SYST, &mut clocks);

    led.toggle(); //checking various steps of the program
    
    let usb_bus = hal::usb_allocator
                    (peripherals.USB, 
                    &mut clocks, 
                    &mut peripherals.PM, 
                    pins.usb_dm,
                    pins.usb_dp,
                    &mut pins.port,
                    );

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    let i2c = hal::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM4, 
        &mut peripherals.PM, 
        pins.sda,
        pins.scl,
        &mut pins.port,
        );  

    let mut val: u8 = 0;

    loop {
 
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }       
        
        let mut text_buf = ArrayString::<[u8; 16]>::new();

        fmt_output(&mut text_buf, val);

        serial.write(text_buf.as_bytes());

        delay.delay_ms(200u16);

        led.toggle();

        val += 1;

    }

}


pub fn fmt_output(buf: &mut ArrayString<[u8; 16]>, val: u8) {   
    
    fmt::write(buf, format_args!("val: {:03}\n", val)).unwrap();

}
