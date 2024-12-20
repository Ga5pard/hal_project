#![no_std]
#![no_main]

mod gpio;
mod usart;
mod spi;
mod i2c;

#[cfg(target_arch = "avr")]
mod atmega;

#[cfg(target_arch = "arm")]
mod cortex_m;

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;

use gpio::GPIO;
use usart::USART;
use spi::SPI;
use i2c::I2C;

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    // ARM example (stub)
    USART::init(9600, false);
    SPI::init_master(0x01);
    I2C::init(100_000); // 100kHz I2C
    loop {}
}

#[cfg(target_arch = "avr")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // AVR example:
    // - Toggle an LED on PB5
    // - Send characters over USART
    // - Send/receive data over SPI
    // - Initialize I2C and perform a start/write transaction

    USART::init(9600, false);
    SPI::init_master(0x01);
    I2C::init(100_000); // I2C at 100kHz

    let led = GPIO::new(5, true).expect("Invalid LED pin");

    // Example I2C usage:
    // Imagine we have a device at address 0x50
    I2C::start();
    I2C::write(0x50 << 1); // Address + write bit
    I2C::write(0x00); // For example, write to register 0x00
    I2C::stop();

    loop {
        // Turn LED on, send 'H' over USART and SPI
        led.set_high();
        USART::write(b'H');
        SPI::transfer(b'H');
        for _ in 0..1_000_000 {}

        // Turn LED off, send 'L' over USART and SPI
        led.set_low();
        USART::write(b'L');
        SPI::transfer(b'L');
        for _ in 0..1_000_000 {}
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
