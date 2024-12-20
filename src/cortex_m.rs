// Stubs for the ARM Cortex-M target.
// These functions do nothing as we don't have actual registers or initialization procedures.
// They exist to show that the code is multi-target capable.

// GPIO
pub fn config_pin(_pin: u8, _output: bool) {}
pub fn write_pin(_pin: u8, _high: bool) {}
pub fn read_pin(_pin: u8) -> bool { false }

// USART
pub fn usart_init(_baud_rate: u16, _double_speed: bool) {}
pub fn usart_write(_data: u8) {}
pub fn usart_read() -> u8 { 0 }

// SPI
pub fn spi_init_master(_clock_div: u8) {}
pub fn spi_init_slave() {}
pub fn spi_transfer(_data: u8) -> u8 { 0 }
pub fn spi_receive() -> u8 { 0 }

// I2C (TWI)
pub fn i2c_init(_frequency: u32) {}
pub fn i2c_start() -> bool { true }
pub fn i2c_stop() {}
pub fn i2c_write(_byte: u8) -> bool { true }
pub fn i2c_read(_ack: bool) -> u8 { 0 }
