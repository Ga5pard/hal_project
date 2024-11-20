#![no_std]
#![no_main]

use cortex_m_rt::entry;

// Importation du module GPIO
mod gpio;
use gpio::GPIO;

#[entry]
fn main() -> ! {
    unsafe {
        // Crée une instance de GPIO pour la broche 5 et la configure en sortie
        if let Some(led) = GPIO::new(5, true) {
            loop {
                led.set_high();
                for _ in 0..1_000_000 {}

                led.set_low();
                for _ in 0..1_000_000 {}
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
