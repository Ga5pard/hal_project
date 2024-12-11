#![no_std]
#![no_main]

mod gpio;
mod usart;
mod spi;
#[cfg(target_arch = "avr")]
mod atmega;

#[cfg(target_arch = "arm")]
mod cortex_m;

use cortex_m_rt::entry;
use gpio::GPIO;
use usart::USART;
use spi::SPI;

#[entry]
fn main() -> ! {
    unsafe {
        // Initialize USART at 9600 baud
        USART::init(9600);

        // Initialize SPI in master mode
        SPI::init_master(0x01);

        // Initialize GPIO pin 5 as output
        if let Some(led) = GPIO::new(5, true) {
            loop {
                // Turn on the LED and send 'H' via USART and SPI
                led.set_high();
                USART::write(b'H');
                SPI::transfer(b'H');
                for _ in 0..1_000_000 {}

                // Turn off the LED and send 'L' via USART and SPI
                led.set_low();
                USART::write(b'L');
                SPI::transfer(b'L');
                for _ in 0..1_000_000 {}
            }
        }

        #[cfg(target_arch = "arm")]
        {
            // Example for Cortex-M
            USART::write(b'A');
            SPI::transfer(b'A');
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}