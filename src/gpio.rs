// gpio.rs

// Déclarations des adresses des registres pour le port B de l'Atmega328p
const DDRB: *mut u8 = 0x24 as *mut u8;  // Registre de direction (entrée/sortie) pour PORTB
const PORTB: *mut u8 = 0x25 as *mut u8; // Registre de données pour définir l'état des broches (HIGH/LOW)
const PINB: *const u8 = 0x23 as *const u8; // Registre pour lire l'état des broches configurées en entrée

/// Structure représentant une broche GPIO.
pub struct GPIO {
    pub pin: u8, // Le numéro de la broche (0 à 7) correspondant à PORTB
}

impl GPIO {
    pub unsafe fn new(pin: u8, output: bool) -> Option<Self> {
        if pin > 7 {
            return None;
        }
        config_pin(pin, output);
        Some(GPIO { pin })
    }

    pub unsafe fn set_mode(&self, output: bool) {
        config_pin(self.pin, output);
    }

    pub unsafe fn set_high(&self) {
        write_pin(self.pin, true);
    }

    pub unsafe fn set_low(&self) {
        write_pin(self.pin, false);
    }

    pub unsafe fn is_high(&self) -> bool {
        read_pin(self.pin)
    }
}

unsafe fn config_pin(pin: u8, output: bool) {
    if pin > 7 {
        return;
    }
    if output {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin));
    } else {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
}

unsafe fn write_pin(pin: u8, high: bool) {
    if pin > 7 {
        return;
    }
    if high {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
    } else {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
    }
}

unsafe fn read_pin(pin: u8) -> bool {
    if pin > 7 {
        return false;
    }
    (core::ptr::read_volatile(PINB) & (1 << pin)) != 0
}
