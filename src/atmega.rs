// atmega.rs

// Registres pour les GPIOs sur l'Atmega328p
pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PINB: *const u8 = 0x23 as *const u8;

// Registres pour l'USART sur l'Atmega328p
pub const UBRRH: *mut u8 = 0x40 as *mut u8;
pub const UBRRL: *mut u8 = 0x29 as *mut u8;
pub const UCSRB: *mut u8 = 0x2A as *mut u8;
pub const UCSRC: *mut u8 = 0x40 as *mut u8;

/// Configure une broche comme entrée ou sortie.
pub unsafe fn config_pin(pin: u8, output: bool) {
    if output {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin));
    } else {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
}

/// Change l'état d'une broche GPIO.
pub unsafe fn write_pin(pin: u8, high: bool) {
    if high {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
    } else {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
    }
}

/// Lit l'état d'une broche GPIO.
pub unsafe fn read_pin(pin: u8) -> bool {
    (core::ptr::read_volatile(PINB) & (1 << pin)) != 0
}

/// Initialise l'USART avec une vitesse en bauds donnée.
pub unsafe fn usart_init(baud_rate: u16) {
    core::ptr::write_volatile(UBRRH, (baud_rate >> 8) as u8);
    core::ptr::write_volatile(UBRRL, baud_rate as u8);
    core::ptr::write_volatile(UCSRB, 1 << 3 | 1 << 4); // RX et TX activés
}

/// Envoie un octet via l'USART.
pub unsafe fn usart_write(data: u8) {
    while core::ptr::read_volatile(UCSRB) & (1 << 5) == 0 {} // Attente de l'envoi
    core::ptr::write_volatile(UCSRB, data);
}

/// Reçoit un octet via l'USART.
pub unsafe fn usart_read() -> u8 {
    while core::ptr::read_volatile(UCSRB) & (1 << 7) == 0 {} // Attente de la réception
    core::ptr::read_volatile(UCSRB)
}
