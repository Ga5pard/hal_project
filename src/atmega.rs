// This file contains the target-specific definitions and implementations for the Atmega328p.

// GPIO
pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PINB: *const u8 = 0x23 as *const u8;

// USART Registers
pub const UBRRH: *mut u8 = 0x40 as *mut u8;
pub const UBRRL: *mut u8 = 0x29 as *mut u8;
pub const UCSRA: *mut u8 = 0x2B as *mut u8;
pub const UCSRB: *mut u8 = 0x2A as *mut u8;
pub const UCSRC: *mut u8 = 0x40 as *mut u8;
pub const UDR:   *mut u8 = 0x2C as *mut u8;

// USART bit definitions
const RXC: u8 = 7;
const UDRE: u8 = 5;
const U2X: u8 = 1;
const RXEN: u8 = 4;
const TXEN: u8 = 3;
const URSEL: u8 = 7;
const UCSZ1: u8 = 2;
const UCSZ0: u8 = 1;

// SPI Registers
pub const SPCR: *mut u8 = 0x2C as *mut u8;
pub const SPSR: *mut u8 = 0x2D as *mut u8;
pub const SPDR: *mut u8 = 0x2E as *mut u8;

// SPI bit definitions
const SPE: u8 = 6;
const MSTR: u8 = 4;
const SPIF: u8 = 7; 

// I2C (TWI) Registers
pub const TWBR: *mut u8 = 0x20 as *mut u8; 
pub const TWSR: *mut u8 = 0x21 as *mut u8; 
pub const TWAR: *mut u8 = 0x22 as *mut u8; 
pub const TWDR: *mut u8 = 0x23 as *mut u8; 
pub const TWCR: *mut u8 = 0x56 as *mut u8; 

// TWI bit definitions
const TWINT: u8 = 7;
const TWEA:  u8 = 6;
const TWSTA: u8 = 5;
const TWSTO: u8 = 4;
const TWEN:  u8 = 2;

// Assuming a CPU frequency of 16 MHz for Atmega328p
const F_CPU: u32 = 16_000_000;

// GPIO Functions
/// Configure a pin on Port B as input or output.
pub fn config_pin(pin: u8, output: bool) {
    if pin > 7 {
        panic!("Invalid pin number: {}. Must be between 0 and 7.", pin);
    }
    unsafe {
        let old_ddr = core::ptr::read_volatile(DDRB);
        let new_ddr = if output {
            old_ddr | (1 << pin)
        } else {
            old_ddr & !(1 << pin)
        };
        core::ptr::write_volatile(DDRB, new_ddr);
    }
}

/// Write a HIGH or LOW level to a specified pin on Port B.
pub fn write_pin(pin: u8, high: bool) {
    if pin > 7 {
        panic!("Invalid pin number: {}. Must be between 0 and 7.", pin);
    }
    unsafe {
        let old_port = core::ptr::read_volatile(PORTB);
        let new_port = if high {
            old_port | (1 << pin)
        } else {
            old_port & !(1 << pin)
        };
        core::ptr::write_volatile(PORTB, new_port);
    }
}

/// Read the state of a pin on Port B.
pub fn read_pin(pin: u8) -> bool {
    if pin > 7 {
        panic!("Invalid pin number: {}. Must be between 0 and 7.", pin);
    }
    unsafe {
        (core::ptr::read_volatile(PINB) & (1 << pin)) != 0
    }
}

/// Initialize the USART with the given baud rate and optional double-speed mode.
pub fn usart_init(baud_rate: u16, double_speed: bool) {
    unsafe {
        core::ptr::write_volatile(UBRRH, (baud_rate >> 8) as u8);
        core::ptr::write_volatile(UBRRL, baud_rate as u8);

        let ucsrb_val = (1 << RXEN) | (1 << TXEN);
        core::ptr::write_volatile(UCSRB, ucsrb_val);

        let ucsrc_val = (1 << URSEL) | (1 << UCSZ1) | (1 << UCSZ0);
        core::ptr::write_volatile(UCSRC, ucsrc_val);

        let old_ucsra = core::ptr::read_volatile(UCSRA);
        let new_ucsra = if double_speed {
            old_ucsra | (1 << U2X)
        } else {
            old_ucsra & !(1 << U2X)
        };
        core::ptr::write_volatile(UCSRA, new_ucsra);
    }
}

/// Write a single byte via USART.
pub fn usart_write(data: u8) {
    unsafe {
        // Wait until UDRE=1 (transmit buffer empty)
        while (core::ptr::read_volatile(UCSRA) & (1 << UDRE)) == 0 {}
        core::ptr::write_volatile(UDR, data);
    }
}

/// Read a single byte from USART.
pub fn usart_read() -> u8 {
    unsafe {
        // Wait until RXC=1 (data received)
        while (core::ptr::read_volatile(UCSRA) & (1 << RXC)) == 0 {}
        core::ptr::read_volatile(UDR)
    }
}

/// Initialize SPI in master mode with the given clock divider.
pub fn spi_init_master(clock_div: u8) {
    unsafe {
        let mut spcr_val = (1 << SPE) | (1 << MSTR);
        spcr_val |= clock_div & 0x03; 
        core::ptr::write_volatile(SPCR, spcr_val);
    }
}

/// Initialize SPI in slave mode.
pub fn spi_init_slave() {
    unsafe {
        let spcr_val = (1 << SPE);
        core::ptr::write_volatile(SPCR, spcr_val);
    }
}

/// Transfer one byte via SPI and return the received byte.
pub fn spi_transfer(data: u8) -> u8 {
    unsafe {
        core::ptr::write_volatile(SPDR, data);
        while (core::ptr::read_volatile(SPSR) & (1 << SPIF)) == 0 {}
        core::ptr::read_volatile(SPDR)
    }
}

/// Receive one byte via SPI (by performing a dummy transfer).
pub fn spi_receive() -> u8 {
    spi_transfer(0xFF)
}

/// Initialize the I2C (TWI) bus with a given frequency (e.g., 100000 for 100kHz).
pub fn i2c_init(frequency: u32) {
    let twbr_val = ((F_CPU / frequency) - 16) / 2;
    unsafe {
        // Prescaler = 1 (TWSR = 0)
        core::ptr::write_volatile(TWSR, 0x00);
        core::ptr::write_volatile(TWBR, twbr_val as u8);
    }
}

/// Send a START condition.
pub fn i2c_start() -> bool {
    unsafe {
        core::ptr::write_volatile(TWCR, (1 << TWINT) | (1 << TWSTA) | (1 << TWEN));
        while (core::ptr::read_volatile(TWCR) & (1 << TWINT)) == 0 {}
    }
    true
}

/// Send a STOP condition.
pub fn i2c_stop() {
    unsafe {
        core::ptr::write_volatile(TWCR, (1 << TWINT) | (1 << TWEN) | (1 << TWSTO));
    }
}

/// Write one byte on the I2C bus.
pub fn i2c_write(byte: u8) -> bool {
    unsafe {
        core::ptr::write_volatile(TWDR, byte);
        core::ptr::write_volatile(TWCR, (1 << TWINT) | (1 << TWEN));
        while (core::ptr::read_volatile(TWCR) & (1 << TWINT)) == 0 {}
    }
    true
}

/// Read one byte from the I2C bus. If ack=true, sends an ACK, otherwise sends a NACK.
pub fn i2c_read(ack: bool) -> u8 {
    unsafe {
        let ctrl = if ack {
            (1 << TWINT) | (1 << TWEN) | (1 << TWEA)
        } else {
            (1 << TWINT) | (1 << TWEN)
        };
        core::ptr::write_volatile(TWCR, ctrl);
        while (core::ptr::read_volatile(TWCR) & (1 << TWINT)) == 0 {}
        core::ptr::read_volatile(TWDR)
    }
}
