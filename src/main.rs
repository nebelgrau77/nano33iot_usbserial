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
        };

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};


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

    led.toggle(); //checking various steps of the program
        
    loop {
 
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }       
        

        serial.write("hello world!".as_bytes()).unwrap();

        //delay.delay_ms(50u16);

        led.toggle();

    }

}
