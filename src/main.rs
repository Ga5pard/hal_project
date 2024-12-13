#![no_std]
#![no_main]

mod gpio;
mod usart;
#[cfg(target_arch = "avr")]
mod atmega;

#[cfg(target_arch = "arm")]
mod cortex_m;

use cortex_m_rt::entry;
use gpio::GPIO;
use usart::USART;

#[entry]
fn main() -> ! {
    unsafe {
        // Initialise l'USART à 9600 bauds
        USART::init(9600);

        // Initialise la broche GPIO 5 comme sortie
        if let Some(led) = GPIO::new(5, true) {
            loop {
                // Allume la LED et envoie 'H' via USART
                led.set_high();
                USART::write(b'H');
                for _ in 0..1_000_000 {}

                // Éteint la LED et envoie 'L' via USART
                led.set_low();
                USART::write(b'L');
                for _ in 0..1_000_000 {}
            }
        }
    }

/* [CORRECTION USART] 

When you are using 'unsafe{}', it is more pertinent to target precisely the emplacement where you do unsafe action.
If you just put all your code in it, its precision advantage does not exist anymore.

(Don't hesitate to remove this comment)
*/

    
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
