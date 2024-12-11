// Registers for GPIOs on the Atmega328p
pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PINB: *const u8 = 0x23 as *const u8;

// Registers for USART on the Atmega328p
pub const UBRRH: *mut u8 = 0x40 as *mut u8;
pub const UBRRL: *mut u8 = 0x29 as *mut u8;
pub const UCSRB: *mut u8 = 0x2A as *mut u8;
pub const UCSRC: *mut u8 = 0x40 as *mut u8;

// Registers for SPI on the Atmega328p
pub const SPCR: *mut u8 = 0x2C as *mut u8;
pub const SPSR: *mut u8 = 0x2D as *mut u8;
pub const SPDR: *mut u8 = 0x2E as *mut u8;

/// Configures a pin as input or output.
pub unsafe fn config_pin(pin: u8, output: bool) {
    if output {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin));
    } else {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
}

/// Changes the state of a GPIO pin.
pub unsafe fn write_pin(pin: u8, high: bool) {
    if high {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
    } else {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
    }
}

/// Reads the state of a GPIO pin.
pub unsafe fn read_pin(pin: u8) -> bool {
    (core::ptr::read_volatile(PINB) & (1 << pin)) != 0
}

/// Initializes USART with a given baud rate.
pub unsafe fn usart_init(baud_rate: u16) {
    core::ptr::write_volatile(UBRRH, (baud_rate >> 8) as u8);
    core::ptr::write_volatile(UBRRL, baud_rate as u8);
    core::ptr::write_volatile(UCSRB, 1 << 3 | 1 << 4); // Enable RX and TX
}

/// Sends a byte via USART.
pub unsafe fn usart_write(data: u8) {
    while core::ptr::read_volatile(UCSRB) & (1 << 5) == 0 {} // Wait for transmission
    core::ptr::write_volatile(UCSRB, data);
}

/// Receives a byte via USART.
pub unsafe fn usart_read() -> u8 {
    while core::ptr::read_volatile(UCSRB) & (1 << 7) == 0 {} // Wait for reception
    core::ptr::read_volatile(UCSRB)
}

/// Initializes SPI in master mode with a given clock divider.
pub unsafe fn spi_init_master(clock_div: u8) {
    core::ptr::write_volatile(SPCR, 1 << 6 | 1 << 4 | clock_div); // Enable SPI, Master mode, set clock divider
}

/// Sends and receives a byte via SPI.
pub unsafe fn spi_transfer(data: u8) -> u8 {
    core::ptr::write_volatile(SPDR, data);
    while core::ptr::read_volatile(SPSR) & (1 << 7) == 0 {} // Wait for transmission to complete
    core::ptr::read_volatile(SPDR)
}